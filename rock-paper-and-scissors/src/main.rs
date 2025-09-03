use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;

mod display_splash_screen;

// Funções para as telas
fn display_main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let mut selected_option = 0;
    let options = [
        "                                        NEW GAME",
        "                                        EXIT",
    ];
    let title = [
        "                            ▄████████  ▄██████▄    ▄████████   ▄█   ▄█▄",
        "                            ███    ███ ███    ███ ███    ███  ███ ▄███▀",
        "                            ███    ███ ███    ███ ███    █▀   ███▐██▀",
        "                           ▄███▄▄▄▄██▀ ███    ███ ███         █████▀",
        "                          ▀▀███▀▀▀▀▀   ███    ███ ███        ▀█████▄",
        "                          ▀███████████ ███    ███ ███    █▄   ███▐██▄",
        "                            ███    ███ ███    ███ ███    ███  ███ ▀███▄",
        "                            ███    ███  ▀██████▀  ████████▀   ███   ▀█▀",
        "",
        "                  ▄███████▄    ▄████████    ▄███████▄     ▄████████    ▄████████",
        "                  ███    ███   ███    ███   ███    ███   ███    ███   ███    ███",
        "                  ███    ███   ███    ███   ███    ███   ███    █▀    ███    ███",
        "                  ███    ███   ███    ███   ███    ███  ▄███▄▄▄      ▄███▄▄▄▄██▀",
        "                ▀█████████▀  ▀███████████ ▀█████████▀  ▀▀███▀▀▀     ▀▀███▀▀▀▀▀",
        "                  ███          ███    ███   ███          ███    █▄  ▀███████████",
        "                  ███          ███    ███   ███          ███    ███   ███    ███",
        "                 ▄████▀        ███    █▀   ▄████▀        ██████████   ███    ███",
        "",
        "     ▄████████  ▄████████  ▄█     ▄████████   ▄████████   ▄██████▄     ▄████████    ▄████████",
        "    ███    ███ ███    ███ ███    ███    ███  ███    ███  ███    ███   ███    ███   ███    ███",
        "    ███    █▀  ███    █▀  ███▌   ███    █▀   ███    █▀   ███    ███   ███    ███   ███    █▀",
        "    ███        ███        ███▌   ███         ███         ███    ███  ▄███▄▄▄▄██▀   ███",
        "  ▀███████████ ███        ███▌ ▀███████████ ▀███████████ ███    ███ ▀▀███▀▀▀▀▀    ▀███████████",
        "           ███ ███    █▄  ███           ███          ███ ███    ███ ▀███████████           ███",
        "      ▄█   ███ ███    ███ ███     ▄█    ███    ▄█    ███ ███    ███   ███    ███     ▄█    ███",
        "   ▄████████▀  ████████▀  █▀    ▄████████▀   ▄████████▀  ▀██████▀    ███    ███   ▄████████▀   ",
        "                                                                                               ",
        "",
        "",
        "> NEW GAME",
        "> EXIT",
    ];

    loop {
        // Limpa a tela
        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 1))?;
        // Exibe o título
        for line in &title[0..29] {
            execute!(stdout, Print(line), Print("\n\r"))?;
        }

        // Exibe as opções do menu
        for (i, &option) in options.iter().enumerate() {
            if i == selected_option {
                // Opção selecionada em verde
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!(" > {}", option)),
                    Print("\n\r")
                )?;
            } else {
                // Outras opções em branco
                execute!(
                    stdout,
                    SetForegroundColor(Color::White),
                    Print(format!("   {}", option)),
                    Print("\n\r")
                )?;
            }
        }
        stdout.flush()?;

        // Captura o evento do teclado
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        if selected_option > 0 {
                            selected_option -= 1;
                        } else {
                            selected_option = options.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_option < options.len() - 1 {
                            selected_option += 1;
                        } else {
                            selected_option = 0;
                        }
                    }
                    KeyCode::Enter => {
                        match selected_option {
                            0 => {
                                // Ação para "NEW GAME"
                                execute!(stdout, Clear(ClearType::All))?;
                                println!("\n\nIniciando um novo jogo...");
                                sleep(Duration::from_secs(2));
                                return Ok(());
                            }
                            1 => {
                                // Ação para "EXIT"
                                execute!(stdout, Clear(ClearType::All))?;
                                println!("\n\nSaindo do jogo...");
                                sleep(Duration::from_secs(1));
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Entra no modo "raw" para capturar as teclas
    enable_raw_mode()?;

    // 1. Exibe a tela de apresentação
    match display_splash_screen::display_splash_screen_a() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Err: {}", err);
        }
    }

    // Limpa a tela
    execute!(stdout(), Clear(ClearType::All))?;
    execute!(stdout(), MoveTo(0, 1))?;

    // 2. Exibe a tela de apresentação
    match display_splash_screen::display_splash_screen_b() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Err: {}", err);
        }
    }

    // Limpa a tela
    execute!(stdout(), Clear(ClearType::All))?;
    execute!(stdout(), MoveTo(0, 1))?;

    // 3. Exibe a tela com "PRESS ANY KEY START GAME"
    match display_splash_screen::display_splash_screen_c() {
        Ok(_) => {}
        Err(err) => {
            eprint!("Err: {}", err);
        }
    }

    // 3. Exibe o menu principal
    display_main_menu()?;

    // Restaura o terminal ao modo normal
    disable_raw_mode()?;

    Ok(())
}
