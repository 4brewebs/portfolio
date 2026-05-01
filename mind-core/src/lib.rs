use wasm_bindgen::prelude::*;
use std::fmt;

// Usamos el raw byte string. Pega TODO tu bloque de texto entre el br#" y el "#.
// He incluido tu lince aquí dentro. Asegúrate de que los saltos de línea 
// se mantienen intactos en tu editor (VS Code).
const LYNX_ASCII_ART: &str = r#"
                                                               ]W   ▒w
                                                               ╓▓  ,▓┘
                                                             ╓███▄██░
                                                        ,▄▄█████████▌
                                                     ,▄███████████████
                                                ,▄████████████████████U
                                              ,████████████████████████
                                            ,▄█████████████████▀▀████▀`
                                          ╓▄██████████████████▒  `▀▀"
                                      ,▄███████████████████████▄╓▄▄▄▄▄,
                                  ,▄████████████████████████████████████
                ▄███▄,        ,▄██████████████████████████████████▀▀▀████▄
                ╙▀"▀▀██▄╖,,╓▄████████████████████████████████▀╙     ╙█████`
                      "▀█████████████████████████████████████▄,       ╙▀▀'
                       ,███████████████████████████╜   `▀███████▄
                      ,█████████████████████████▀`        ▀██████
                      ▓█████████████▀▀▀▀▀▀▀▀"               ╙▀▀▀
                     j████████████▌
                     ╟███████████▌
                    ▐███████████▀
                  ,██████████▀▀
                ▓█████████▀"
               ]███████▌
               ╟███]███C
              ╓███░ ███Ç
              ████▄,▓████∩
              ╙████▌ ▀██▀
               ╙▀▀╙
     
"#;


struct LynxArt;

impl LynxArt {
    fn new() -> Self {
        Self
    }
}

impl fmt::Display for LynxArt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", LYNX_ASCII_ART)
    }
}


#[wasm_bindgen]
pub fn get_welcome_message() -> String {
    let lynx = LynxArt::new();
    
    // Nuestra lógica de replace se asegura de que xterm.js lo pinte sin romper columnas
    let formatted_art = lynx.to_string().replace("\n", "\r\n");

    format!(
        "{}\r\n\
        =======================================\r\n\
        terMindOs Terminal Portfolio 🚀\r\n\
        =======================================\r\n\
        Bienvenido a la terminal profesional.\r\n\
        Escribe 'help' para ver la lista de comandos.\r\n",
        formatted_art
    )
}


// Esta macro le dice al compilador que exporte esta función a JavaScript
#[wasm_bindgen]
pub fn process_command(input: &str) -> String {
    // 1. Limpiamos espacios en blanco y separamos el input por palabras
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Si el usuario solo pulsó Enter sin escribir nada, devolvemos vacío
    if parts.is_empty() {
        return String::new();
    }

    // 2. Extraemos el comando base (en minúsculas)
    let command = parts[0].to_lowercase();
    // let _args = &parts[1..]; // Guardado para futuros comandos con argumentos

    // 3. El enrutador principal de comandos
    match command.as_str() {
        "help" => String::from(
            "Comandos disponibles:\r\n  \
            help     - Muestra este mensaje\r\n  \
            about    - Información sobre mi perfil\r\n  \
            skills   - Stack tecnológico y herramientas\r\n  \
            clear    - Limpia la pantalla de la terminal"
        ),
        
        "about" => String::from(
            "¡Hola! Soy un desarrollador de software enfocado en crear \r\n\
            arquitecturas sólidas y experiencias de usuario excepcionales.\r\n\
            Me encanta el ecosistema de Rust y WebAssembly."
        ),
        
        "skills" => String::from(
            "🚀 Frontend: Svelte, TypeScript, CSS3\r\n\
             🦀 Core/Backend: Rust, WebAssembly\r\n\
             🛠️ Herramientas: Git, GitLab CI/CD, Vite"
        ),
        "clear" => String::new(),
        "ai-chat" => String::new(), // Devolver vacío para que JS sepa limpiar la pantalla

        // Comando secreto/easter egg (¡siempre quedan bien en los portfolios!)
        "sudo" => String::from("buen intento... pero este incidente será reportado. 🕵️‍♂️"),

        // Fallback para cuando el comando no existe
        _ => format!("Comando no encontrado: '{}'. Escribe 'help' para ver los comandos.", command),
    }
}