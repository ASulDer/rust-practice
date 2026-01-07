use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Write};
use Chain::{Top, Bottom};

// ===================
// | STRUCTS & ENUMS |
// ===================

#[derive(PartialEq)]
enum Chain {
    Top,        // Gorny lancuch
    Bottom,     // Dolny lancuch
    Undefined,  // Przynaleznosc do lancuchu nie zdefiniowana
}

struct Vertex {
    x: i32,         // Odcieta
    y: i32,         // Rzędna
    chain: Chain,   // lancuch, do ktorego nalezy wierzcholek
}

struct Diagonal {
    a: usize,       // punkt (wierzcholek) a // wskazane jest indeksami, ale na dobra sprawe nalezy ty uzyc referencji do poszczegolnych vertex'ow
    b: usize,       // punkt (wierzcholek) b // wskazane jest indeksami, ale na dobra sprawe nalezy ty uzyc referencji do poszczegolnych vertex'ow
}

// ==================
// |      MAIN      |
// ==================

fn main() {
    let mut file_names: Vec<String> = Vec::new();
    for argument in env::args().skip(1) {
        file_names.push(argument);
    }

    if file_names.len() == 0 {
        panic!("Please provide full file(s) name(s) as an argument.\nProsze podac pelna nazwe pliku jako argument.\n");
    }

    for file_name in file_names {
        print!("Press any key to start triangulation process.\nNacisnij dowonlna klawisze zeby zaczac proces triangulacji.");
        std::io::stdout().flush().unwrap();
        {
            let mut dummy_buffer = String::new();
            let _ = std::io::stdin().read_line(&mut dummy_buffer);
        }

        let figure_data = File::open(&file_name);
        if figure_data.is_err() {
            println!("Error while reading file {}. Processing the next one.\nBlad przy wczytywaniu pliku z nazwa {}. Przeskakuje na nastepny."
            , file_name, file_name);
            continue;
        }

        let figure_vertexes = ParseFigureData(figure_data.unwrap());
        // for vertex in figure_vertexes {      // Wypisuje posortowane wierzcholki
        //     println!("Vertex: {}, {}", vertex.x, vertex.y);
        // }
        for diag in SweepTriangulation(&figure_vertexes) {
            println!("({}, {}) ({}, {})", figure_vertexes[diag.a].x, figure_vertexes[diag.a].y, figure_vertexes[diag.b].x, figure_vertexes[diag.b].y);
        }
    }
}

// ==================
// | MISC FUNCTIONS |
// ==================

// Tez od razu sortuje wierzcholki wedlug wartosci odcietej
fn ParseFigureData(figure_data: File) -> Vec<Vertex> {
    let mut data_reader = BufReader::new(figure_data);
    let mut buffer_str = String::new();

    data_reader.read_line(&mut buffer_str).expect("Was not able to read data.\nNie dało się przeczytać danych.");
    let n = buffer_str.trim().split_whitespace().next().unwrap().parse().expect("Was not able to parse a number.\nNie dało się sparsować liczby.");

    let mut figure_vertexes = Vec::with_capacity(n);
    {
        buffer_str.clear();
        data_reader.read_line(&mut buffer_str).expect("Was not able to read data.\nNie dało się przeczytać danych.");
        let mut coordinate_iterator = buffer_str.trim().split_whitespace();
        let x = coordinate_iterator.next().unwrap().parse().expect("Was not able to parse a number.\nNie dało się sparsować liczby.");
        let y = coordinate_iterator.next().unwrap().parse().expect("Was not able to parse a number.\nNie dało się sparsować liczby.");
        figure_vertexes.push(Vertex { x, y, chain: Chain::Undefined });
        let mut previous_x = x.clone();
        for _ in 1..n {
            buffer_str.clear();
            data_reader.read_line(&mut buffer_str).expect("Was not able to read data.\nNie dało się przeczytać danych.");
            let mut coordinate_iterator = buffer_str.trim().split_whitespace();
            let x = coordinate_iterator.next().unwrap().parse().expect("Was not able to parse a number.\nNie dało się sparsować liczby.");
            let y = coordinate_iterator.next().unwrap().parse().expect("Was not able to parse a number.\nNie dało się sparsować liczby.");
            if x >= previous_x {
                figure_vertexes.push(Vertex { x, y, chain: Bottom });
            } else {
                figure_vertexes.push(Vertex { x, y, chain: Top });
            }
            previous_x = x.clone();
        }
        if figure_vertexes[0].x >= figure_vertexes[n-1].x {
            figure_vertexes[0].chain = Bottom;
        } else {
            figure_vertexes[0].chain = Top;
        }
    }
    
    figure_vertexes.sort_by_key(|v| v.x);
    figure_vertexes
}

fn SweepTriangulation(figure_vertexes: &Vec<Vertex>) -> Vec<Diagonal> {
    let n = figure_vertexes.len();
    if n < 3 {
        println!("It's a line, not a figure!");
        return vec![];
    }

    let mut diagonals: Vec<Diagonal> = Vec::new();
    // Wlozenie v0 i v1 na stos
    let mut stack: Vec<usize> = Vec::new();
    stack.push(0);
    stack.push(1);

    for i in 2..n - 1 {
        let current_vertex = &figure_vertexes[i];
        let last_vertex = &figure_vertexes[*stack.last().unwrap()];

        if current_vertex.chain == last_vertex.chain {
            let mut last_on_same_chain = stack.pop().unwrap();
            while !stack.is_empty() && Diagonal_IsInside(&figure_vertexes, stack.last().unwrap().clone(), last_on_same_chain, i)
            {
                let last_on_stack = stack.last().unwrap().clone();
                diagonals.push(Diagonal { a: last_on_stack, b: i });
                last_on_same_chain = stack.pop().unwrap();
            }

            stack.push(last_on_same_chain);
            stack.push(i);
        } else {
            let last_vertex_index = stack.last().unwrap().clone();

            for j in 1..stack.len() {
                diagonals.push(Diagonal {a: stack[j], b: i});
            }
                
            // Oproznienie stosu
            stack.clear();
        
            // Wlozenie indeksow wierzcholkow vk i v na stos
            stack.push(last_vertex_index);
            stack.push(i);
        }
    }

    for j in 1..stack.len() - 1 {
        diagonals.push(Diagonal { a: n - 1, b: stack[j] });
    }

    diagonals
}

// Potrzeba we wskazaniu zbioru wszystkich wierzcholkow wynika z tego, że korzystam z indeksow :(
fn Diagonal_IsInside(figure_vertexes: &&Vec<Vertex>, last_on_stack: usize, popped: usize, i: usize) -> bool {
    let v_last  = &figure_vertexes[last_on_stack];
    let v_popped = &figure_vertexes[popped];
    let v_curr = &figure_vertexes[i];

    // Sprawdzenie iloczynu wektorow przekatnych
    match v_curr.chain {
        Chain::Top => {
            (v_popped.x - v_curr.x) * (v_last.y - v_curr.y) - (v_popped.y - v_curr.y) * (v_last.x - v_curr.x) > 0
        }

        Chain::Bottom => {
            (v_popped.x - v_curr.x) * (v_last.y - v_curr.y) - (v_popped.y - v_curr.y) * (v_last.x - v_curr.x) < 0
        }

        _ => false
    }
}