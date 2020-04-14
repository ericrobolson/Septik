use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    build_platform_specific_shaders();
}

fn build_platform_specific_shaders() {
    //NOTE: Shaders are written as 'someshader.vert||frag.src' as they contain all information related to compiling on all platforms.
    // This way, when building, it'll go through the 'src' shader, parse out any irrelevant information for the target platform, then include it.
    // Main thing is stuff like 'precision highp float;' will be dependant on platform, as well as the OpenGL versions.
    // The reason all platform information is included in each src shader is so that there's less chances of mistakes.
    let mut cargo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_path.push("src");

    let mut shaders_path = cargo_path.clone();
    shaders_path.push("backend\\sdl2_gl_pipeline\\shaders");

    // Process base files
    {
        let mut base_shaders_path = shaders_path.clone();
        base_shaders_path.push("base");

        let paths = fs::read_dir(base_shaders_path).unwrap();

        let mut gen_file_path = shaders_path.clone();
        gen_file_path.push("gen");

        for path in paths {
            let path = path.unwrap();

            let src_file = File::open(path.path()).unwrap();

            let mut gen_file;
            {
                let file_name = path.file_name().into_string().unwrap();
                // Get the final generated file name by stripping out the 'src' bit
                let base_fname = file_name.replace(".src", "");

                let mut gen_file_path = gen_file_path.clone();
                gen_file_path.push(base_fname);

                gen_file = File::create(gen_file_path).unwrap();
            }

            // Open and parse source file
            let src_file = BufReader::new(src_file);
            for line in src_file.lines() {
                let ln = line.unwrap();

                // NOTE: This is where you'd strip out certain things to compile the shaders depending on target platform
                writeln!(gen_file, "{}", ln.to_string()).unwrap();
            }
        }
    }
}
