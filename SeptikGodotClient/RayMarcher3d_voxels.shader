// Inspired by https://github.com/PLUkraine/raymarching-godot

shader_type spatial;

// Uniforms //

uniform sampler2D mask_texture;
uniform bool use_texture;

// Camera/viewpoint 
uniform float fov = 45.0; // Vertical FOV
//uniform vec3 cameraPosition = vec3(0.0, 0.0, 5.0);  // The position of the camera in world coordinates
uniform vec3 eye = vec3(0.0, 0.0, -1.0); // Where we're looking at
uniform vec3 up = vec3(0.0, 1.0, 0.0); // The 'up' direction

// Raymarching
uniform int MAX_STEPS = 250; // The max amount of steps to march
uniform float MAX_DISTANCE = 20; // The maximum depth to traverse
uniform float MINIMUM_HIT_DISTANCE = 0.00001; // The minimum depth threshold
uniform float DERIVATIVE_STEP = 0.0001;

// Phong/lighting
uniform float globalAmbient = 0.1; // How strong the ambient lighting is
uniform float globalDiffuse = 1.0; // How strong the diffuse lighting is
uniform float globalSpecular = 1.0; // How strong the specular lighting is
uniform float globalSpecularExponent = 63.0; // How focused the specular 'shine' is
uniform vec3 lightPos = vec3(-2.0, 5.0, 3.0); // The position of the light
uniform vec3 lightColor = vec3(0.9, 0.9, 0.68); // The light's color
uniform vec3 ambientColor = vec3(1.0, 1.0, 1.0); // The ambient light's color

// End uniforms // 

// Functionality //

// Signed distance function for spheres
// Additional SDF's: https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdSphere(vec3 position, vec3 center, float radius){
	return length(center - position) - radius;
}

float sdBox( vec3 position, vec3 b )
{
  vec3 q = abs(position) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}


float sdRoundBox( vec3 p, vec3 b, float r )
{
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0) - r;
}

float sdTexture(vec3 position, vec3 uv){
	//ivec2 texture_size = textureSize(mask_texture, 0);
		
	
	vec4 color = texture(mask_texture, vec2(uv.x, uv.y));
	
	return 0.0;
}


// Signed distance function given a position
float sdf(vec3 pos){		
	
	if (use_texture){
		return sdRoundBox(pos, vec3(1), 1);	
	}
	
	return sdSphere(pos, vec3(0), 1);
}

// Given a resolution and a UV, calculate the ray direction
vec3 getRayDirection(vec2 resolution, vec2 uv){
	float aspect = resolution.x / resolution.y;
	float fov2 = radians(fov) / 2.0;
	
	// Normalize coordinates from [0, 1] to [-1, 1]
	vec2 screenCoordinates = (uv - 0.5) * 2.0;
	screenCoordinates.x *= aspect;
	screenCoordinates.y = -screenCoordinates.y; // Flip y axis, as in godot it goes from top to bottom
	
	// Get the contributions of the up and right vectors
	vec2 offsets = screenCoordinates * tan(fov2);
	
	// Compute the 3 orthogonal unit vectors
	vec3 rayFront = normalize(eye);
	vec3 rayRight = normalize(cross(rayFront, normalize(up)));
	vec3 rayUp = cross(rayRight, rayFront);
	vec3 rayDir = rayFront + rayRight * offsets.x + rayUp * offsets.y;
	
	return normalize(rayDir);	
}

vec3 estimateNormal(vec3 position){
	return normalize(vec3(
		sdf(vec3(position.x + DERIVATIVE_STEP, position.y, position.z)) - sdf(vec3(position.x - DERIVATIVE_STEP, position.y, position.z)),
		sdf(vec3(position.x, position.y + DERIVATIVE_STEP, position.z)) - sdf(vec3(position.x, position.y  - DERIVATIVE_STEP, position.z)),
		sdf(vec3(position.x, position.y, position.z + DERIVATIVE_STEP)) - sdf(vec3(position.x, position.y, position.z - DERIVATIVE_STEP))
	));
}
// End functionality // 


// Lighting //

vec3 blinnPhong(
	vec3 position
	, vec3 lightPosition
	, vec3 ambientCol
	, vec3 lightCol
	, float ambientCoeff
	, float diffuseCoeff
	, float specularCoeff
	, float specularExponent
	){
	vec3 normal = estimateNormal(position);
	vec3 toEye = normalize(eye - position);
	vec3 toLight = normalize(lightPosition - position);
	vec3 reflection = reflect(-toLight, normal);
	
	vec3 ambientFactor = ambientCol * ambientCoeff;
	vec3 diffuseFactor = diffuseCoeff * lightCol * max(0.0, dot(normal, toLight));
	vec3 specularFactor = lightCol * pow(max(0.0, dot(toEye, reflection)), specularExponent) * specularCoeff;
	
	return ambientFactor + diffuseFactor + specularFactor;
}


// End Lighting //


// Given a direction, march forward
vec3 raymarch(vec3 rayDirection, vec3 cameraPosition){
	vec3 hitColor = vec3(1.0, 1.0, 1.0);
	vec3 missColor = vec3(0.0, 0.0, 0.0);
	
	float depth = 0.0;
	for (int i = 0; depth < MAX_DISTANCE && i < MAX_STEPS; ++i){
		vec3 position = cameraPosition + rayDirection * depth;
		float dist = sdf(position);
		if (dist < MINIMUM_HIT_DISTANCE) {
			    return blinnPhong(
					position
					, lightPos
					, ambientColor
					, lightColor
					, globalAmbient
					, globalDiffuse
					, globalSpecular
					, globalSpecularExponent
				);
		}
		
		depth += dist;
	}
	
	return missColor;
}

// The actual shader
void fragment(){	
	vec2 resolution = 1.0 / VIEWPORT_SIZE;
	//resolution = VIEWPORT_SIZE;
	
	vec3 rayDir = getRayDirection(resolution, UV);
	
	vec3 raymarchColor = raymarch(rayDir, VIEW);
	
	// Show direction on screen as a color	
	ALBEDO = raymarchColor;
	ALPHA = 1.0;
}