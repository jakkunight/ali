use crate::utils::input;
use std::{io::{self, Write}, ops::{Add, Mul, Sub}};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    data: Vec<Vec<f32>>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    // INFO: Creación de una matriz de ceros:
    pub fn new(rows: usize, columns: usize) -> Self {
        let data: Vec<Vec<f32>> = vec![vec![0.0; columns]; rows];
        Matrix {
            data,
            rows,
            columns,
        }
    }

    // INFO: Lee las filas y columnas de la matriz:
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    // INFO: Validación de índices:
    fn validate_indexes(&self, row: usize, column: usize) -> bool {
        if row >= self.rows || column >= self.columns {
            return false;
        }
        return true;
    }

    // INFO: Lee un elemento de la matriz:
    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        self.data.get(row)?.get(col).cloned()
    }

    // INFO: Escribe un elemento en la matriz:
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        if let Some(row_vec) = self.data.get_mut(row) {
            if let Some(cell) = row_vec.get_mut(col) {
                *cell = value;
            }
        }
    }

    // NOTE: Cálculo del determinante mediante el método de Laplace:
    pub fn determinant(&self) -> f32 {
        assert_eq!(
            self.rows, self.columns,
            "Matrix must be square to calculate determinant"
        );

        if self.rows == 1 {
            return self.get(0, 0).unwrap();
        }

        let mut det = 0.0;

        for j in 0..self.columns {
            det += self.get(0, j).unwrap() * self.cofactor(0, j);
        }

        det
    }

    // INFO: Cálculo del cofactor de un elemento:
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        let sign = (-1.0_f32).powi((row + col) as i32);
        sign * minor.determinant()
    }

    // INFO: Cálculo del menor correspondiente a un elemento:
    pub fn minor(&self, row: usize, col: usize) -> Matrix {
        let mut minor = Matrix::new(self.rows - 1, self.columns - 1);

        let mut minor_row = 0;
        for i in 0..self.rows {
            if i == row {
                continue;
            }
            let mut minor_col = 0;
            for j in 0..self.columns {
                if j == col {
                    continue;
                }
                minor.set(minor_row, minor_col, self.get(i, j).unwrap());
                minor_col += 1;
            }
            minor_row += 1;
        }

        minor
    }

    // INFO: Muestra la matriz en pantalla:
    pub fn display(&self) {
        for i in 0..self.rows {
            for j in 0..self.columns {
                print!("| {} |", self.get(i, j).unwrap());
            }
            println!();
        }
    }

    // INFO: Permite al usuario cargar los valores en la matriz:
    pub fn load() -> Self {
        let rows = input::<usize>(Some("Insert number of rows:".to_string()));
        let columns = input::<usize>(Some("Insert number of columns:".to_string()));
        let mut m = Matrix::new(rows, columns);
        println!("Load the matrix data. Data is loaded as the \"matrix definition\" says. ");
        for i in 0..m.rows {
            for j in 0..m.columns {
                m.display();
                print!("Value for [ {}, {} ]:", i, j);
                m.set(i, j, input::<f32>(None));
                print!("\x1b[{}A", m.rows + 1);
                for _k in 0..(m.rows + 1) {
                    print!("\x1b[2K");
                }
                io::stdout().flush().expect("ERROR");
            }
        }
        m.display();
        println!();
        m
    }

    // INFO: Permite al usuario modificar los valores en la matriz:
    pub fn edit(&mut self) {
        println!("Load the matrix data. Data is loaded as the \"matrix definition\" says. ");
        for i in 0..self.rows {
            for j in 0..self.columns {
                self.display();
                print!("Value for [ {}, {} ]:", i, j);
                self.set(i, j, input::<f32>(None));
                print!("\x1b[{}A", self.rows + 1);
                for _k in 0..(self.rows + 1) {
                    print!("\x1b[2K");
                }
                io::stdout().flush().expect("ERROR");
            }
        }
        self.display();
        println!();
    }
}

// INFO: Suma de matrices:
impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(
            (self.rows, self.columns),
            (other.rows, other.columns),
            "Matrix dimensions must match for addition"
        );

        let mut result = Matrix::new(self.rows, self.columns);

        for i in 0..self.rows {
            for j in 0..self.columns {
                let sum = self.get(i, j).unwrap() + other.get(i, j).unwrap();
                result.set(i, j, sum);
            }
        }

        result
    }
}

// INFO: Resta de matrices:
impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(
            (self.rows, self.columns),
            (other.rows, other.columns),
            "Matrix dimensions must match for addition"
        );

        let mut result = Matrix::new(self.rows, self.columns);

        for i in 0..self.rows {
            for j in 0..self.columns {
                let sub = self.get(i, j).unwrap() - other.get(i, j).unwrap();
                result.set(i, j, sub);
            }
        }

        result
    }
}

// INFO: Multiplicación de una matriz por un escalar:
impl Mul<f32> for Matrix {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        let mut result = Matrix::new(self.rows, self.columns);

        for i in 0..self.rows {
            for j in 0..self.columns {
                let product = self.get(i, j).unwrap() * scalar;
                result.set(i, j, product);
            }
        }

        result
    }
}

// INFO: Multiplicación de una matriz por matriz:
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // NOTE:
        // self    Matriz A
        // other   Matriz B
        // Chequea si es posible multiplicar las matrices.
        // Si la condición no se cumple, el programa se detiene:
        assert_eq!(
            self.columns,
            other.rows,
            "Number of columns in first matrix must equal number of rows in second matrix for multiplication"
        );

        // NOTE:
        // Procede a multiplicar las matrices.
        // Crea la matriz resultado:
        let mut result = Matrix::new(self.rows, other.columns);

        // NOTE: Para cada entrada de la matriz resultante calcula su valor:
        for i in 0..self.rows {
            for j in 0..other.columns {
                let mut sum = 0.0;
                for k in 0..self.columns {
                    sum += self.get(i, k).unwrap() * other.get(k, j).unwrap();
                }
                result.set(i, j, sum);
            }
        }

        result
    }
}


