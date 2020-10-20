use std::io::{ stdin, stdout, Write };
use std::{ thread, time };
use std::process::Command;
const TITULO: &str = "App data e hora";

/*
 * Faça um app data. Ele deve inicializar o tempo
 * lendo o dia, mês, ano, e as horas. Por fim, o
 * tempo deve ser mostrado passando, atualizando
 * as horas e datas
 */

fn main() {
    clear_terminal();

    let mut segundos: i32 = 0;
    ler_segundos(&mut segundos);
    let mut minutos: i32 = 0;
    ler_minutos(&mut minutos);
    let mut horas: i32 = 0;
    ler_horas(&mut horas);

    let mut ano: i32 = 1;
    ler_ano(&mut ano);
    let ano_bissexto: bool = eh_ano_bissexto(ano);

    let mut mes: i32 = 1;
    ler_mes(&mut mes);
    let maximo_de_dias: i32 = maximo_de_dias_no_mes_e_tipo_do_ano(mes, ano_bissexto);

    let mut dia: i32 = 1;
    ler_dia(&mut dia, maximo_de_dias);

    let mut duracao_do_app: i64 = 0;
    ler_duracao_do_app(&mut duracao_do_app);
    let mut data_e_hora_inicial: [i32; 6] = [dia, mes, ano, horas, minutos, segundos];
    iniciar_a_passagem_de_tempo(duracao_do_app, &mut data_e_hora_inicial);

    print!("\n\nPrograma finalizado.\n");
    print!("Até mais!\n");
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}

fn mostrar_header_do_programa() {
    print!("\n\t{}\n\n", TITULO);
}

fn ler_variavel_i32(tempo: &mut i32) {
    print!(">>> "); stdout().flush().expect("escrita!");

    let mut tempo_como_string = String::new();
    stdin().read_line(&mut tempo_como_string).expect("leitura!");
    *tempo = tempo_como_string.trim().parse::<i32>().unwrap_or(-1);
    tempo_como_string.clear();
}

fn ler_variavel_i64(tempo: &mut i64) {
    print!(">>> "); stdout().flush().expect("escrita!");

    let mut tempo_como_string = String::new();
    stdin().read_line(&mut tempo_como_string).expect("leitura!");
    *tempo = tempo_como_string.trim().parse::<i64>().unwrap_or(-1);
    tempo_como_string.clear();
}

fn ler_segundos(segundos: &mut i32) {
    loop {
        mostrar_header_do_programa();
        print!("\nInicialize os segundos:\n");
        ler_variavel_i32(segundos);
        clear_terminal();
        if segundos_sao_validos(*segundos) {
            break;
        } else {print!("Dê segundos válidos!");}
    }
}

fn segundos_sao_validos(segundos: i32) -> bool {
    if segundos >= 0 && segundos < 60 { true }  else { false }
}

fn ler_minutos(minutos: &mut i32) {
    loop {
        mostrar_header_do_programa();
        print!("\nInicialize os minutos:\n");
        ler_variavel_i32(minutos);
        clear_terminal();
        if minutos_sao_validos(*minutos) {
            break;
        } else {print!("Dê minutos válidos!");}
    }
}

fn minutos_sao_validos(minutos: i32) -> bool {
    if minutos >= 0 && minutos < 60 { true } else { false }
}

fn ler_horas(horas: &mut i32) {
    loop {
        mostrar_header_do_programa();
        print!("\nInicialize as horas:\n");
        ler_variavel_i32(horas);
        clear_terminal();
        if horas_sao_validas(*horas) {
            break;
        } else {print!("Dê horas válidas!");}
    }
}

fn horas_sao_validas(horas: i32) -> bool {
    if horas >= 0 && horas < 24 { true } else { false }
}

fn ler_ano(ano: &mut i32) {
    loop {
        mostrar_header_do_programa();
        print!("\nInicialize o ano:\n");
        ler_variavel_i32(ano);
        clear_terminal();
        if ano_eh_valido(*ano) {
            break;
        } else {print!("Esse ano não é válido!");}
    }
}

fn ano_eh_valido(ano: i32) -> bool {
    if ano >= 1 { true } else { false }
}

fn eh_ano_bissexto(ano: i32) -> bool {
    if ano < 100 { (ano % 4) == 0 }
    else { ((ano%4) == 0) || (((ano%100) == 0) && ((ano%400) == 0)) }
}

fn ler_mes(mes: &mut i32) {
    loop {
        mostrar_header_do_programa();
        print!("\nInicialize o mês:\n");
        ler_variavel_i32(mes);
        clear_terminal();
        if mes_eh_valido(*mes) {
            break;
        } else {print!("Esse mês não é válido!");}
    }
}

fn mes_eh_valido(mes: i32) -> bool {
    if mes >= 1 && mes <= 12 { true } else { false }
}

fn ler_dia(dia: &mut i32, maximo_de_dias: i32) {
    loop {
        mostrar_header_do_programa();
        print!("\nInicialize o dia:\n");
        ler_variavel_i32(dia);
        clear_terminal();
        if *dia <= maximo_de_dias {
            break;
        } else {print!("Esse dia não é válido!");}
    }
}

fn maximo_de_dias_no_mes_e_tipo_do_ano(mes: i32, eh_ano_bissexto: bool) -> i32 {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {if eh_ano_bissexto { 29 } else { 28 }}
        _ => {print!("O mês dado não é válido!"); -1}
    }
}

fn ler_duracao_do_app(tempo: &mut i64) {
    loop {
        mostrar_header_do_programa();
        print!("\nPor quantos segundos o app funcionará?\n");
        print!("Caso seja zero, o app parará apenas com CTRL+C.\n");
        ler_variavel_i64(tempo);
        clear_terminal();
        if *tempo >= 0 {
            break;
        } else {print!("Dê um tempo válido");}
    }
}

fn iniciar_a_passagem_de_tempo(duracao: i64, partida: &mut [i32; 6]) {
    if duracao < 1 {
        relogio_sem_parar(partida);
    } else {
        relogio_com_parada(duracao, partida);
    }
}

fn relogio_sem_parar(partida: &mut [i32; 6]) {
    let um_segundo = time::Duration::from_millis(1000);
    loop {
        clear_terminal();
        mostrar_header_do_programa();
        mostrar_tempo_formatado(partida);
        thread::sleep(um_segundo);
    }
}

fn relogio_com_parada(tempo: i64, partida:  &mut [i32; 6]) {
    let duracao_do_app = time::Duration::from_millis((tempo as u64)*1000);
    let um_segundo = time::Duration::from_millis(1000);
    let a_partir_de_agora = time::Instant::now();
    while a_partir_de_agora.elapsed() <= duracao_do_app {
        clear_terminal();
        mostrar_header_do_programa();
        mostrar_tempo_formatado(partida);
        thread::sleep(um_segundo);
    }
}

fn mostrar_tempo_formatado(partida: &mut [i32; 6]) {
    let dia = partida[0];
    let mes = partida[1];
    let ano = partida[2];
    let horas = partida[3];
    let minutos = partida[4];
    let segundos = partida[5];
    let ano_bissexto: bool = eh_ano_bissexto(ano);

    if dia < 10 { print!("0{}:", dia); }
        else { print!("{}:", dia); }
    if mes < 10 { print!("0{}:", mes); }
        else { print!("{}:", mes); }
    print!("{} - ", ano);

    if horas < 10 { print!("0{}:", horas); }
        else { print!("{}:", horas); }
    if minutos < 10 { print!("0{}:", minutos); }
        else { print!("{}:", minutos); }
    if segundos < 10 { print!("0{}", segundos); }
        else { print!("{}", segundos); }
    stdout().flush().expect("escrita!");

    partida[5] = if segundos < 59 { segundos + 1 } else {
        partida[4] = if minutos < 59 { minutos + 1 } else {
            partida[3] = if horas < 23 { horas + 1 } else {
                partida[0] = if dia < maximo_de_dias_no_mes_e_tipo_do_ano(mes, ano_bissexto) { dia + 1 } else {
                    partida[1] = if mes < 12 { mes + 1 } else {
                        partida[2] += 1;
                    1 };
                1 };
            0 };
        0 };
    0 };
}
