fn main() {
    // Recompile shaders if we change them.
    println!("cargo:rerun-if-changed=res/shader/*");
     
    // Use shaderc to build the shaders.
    let mut compiler = shaderc::Compiler::new().unwrap();
    
    for entry in std::fs::read_dir("res/shader").unwrap() {
        let path = entry.unwrap().path();

        let file_name = &path.file_name().unwrap().to_str().unwrap();

        if !file_name.ends_with(".spv") {
            let folder = &path.parent().unwrap();
            let source = std::fs::read_to_string(&path).unwrap();

            let spirv = {
                if file_name.ends_with(".frag") {
                    compiler.compile_into_spirv(
                        &source,
                        shaderc::ShaderKind::Fragment,
                        file_name,
                        "main",
                        None
                    ).unwrap()
                } else {
                    compiler.compile_into_spirv(
                        &source,
                        shaderc::ShaderKind::Vertex,
                        file_name,
                        "main",
                        None
                    ).unwrap()
                }
            };

            let new_path = folder.join(format!("{}.spv", file_name));

            std::fs::write(new_path, spirv.as_binary_u8());
            
        }

        
    }
}