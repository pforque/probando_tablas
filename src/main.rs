use eframe::run_native;
use eframe::{
    egui::{CentralPanel, Direction, Layout},
    App, NativeOptions, Theme,
};
use egui::{SidePanel, Vec2};
use egui_extras::{Column, TableBuilder};
use native_dialog::{MessageDialog, MessageType};
use sqlite::Error as SqliteError;
use sqlite::State;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Genero {
    Masculino,
    Femenino,
    Otro,
}

impl fmt::Display for Genero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

struct Test {
    vector: Vec<[String; 6]>,
    nombre: String,
    apellidos: String,
    genero: Genero,
    nacionalidad: String,
    fecha: String,
    connection: sqlite::Connection,
    is_table_created: bool,
    id: i32,
    contador: i32,
    int_option: usize,
}

impl Default for Test {
    fn default() -> Self {
        let vector = Vec::with_capacity(1000);
        Self {
            vector,
            nombre: "".to_string(),
            apellidos: "".to_string(),
            genero: Genero::Masculino,
            nacionalidad: "".to_string(),
            fecha: "".to_string(),
            connection: get_connection(),
            is_table_created: false,
            id: 1,
            contador: 0,
            int_option: 0,
        }
    }
}

impl App for Test {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.is_table_created.eq(&false) {
            create_table(&self.connection);
            self.is_table_created = true;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Tema", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
        });

        SidePanel::left("side_panel")
            .default_width(234.0)
            .min_width(234.0)
            .max_width(234.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Apellidos:");
                ui.text_edit_singleline(&mut self.apellidos);
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.nombre);
                ui.label("Sexo:");
                egui::ComboBox::from_label(String::new())
                    .selected_text(self.genero.to_string())
                    .show_ui(ui, |ui| {
                        for genero in [Genero::Masculino, Genero::Femenino, Genero::Otro] {
                            ui.selectable_value(&mut self.genero, genero, genero.to_string());
                        }
                    });
                ui.label("Nacionalidad:");
                ui.text_edit_singleline(&mut self.nacionalidad);
                ui.label("Fecha nacimiento:");
                ui.text_edit_singleline(&mut self.fecha);
                ui.horizontal(|ui| {
                    if ui.button("Alta").clicked()
                        && validation(
                            &self.nombre,
                            &self.apellidos,
                            &self.nacionalidad,
                            &self.fecha,
                        )
                    {
                        let _usuario = crear_usuario(
                            self.id,
                            self.apellidos.clone(),
                            self.nombre.clone(),
                            self.genero.to_string(),
                            self.nacionalidad.clone(),
                            self.fecha.clone(),
                        );
                        _usuario.insert_into_db(&self.connection).unwrap();
                        self.contador = count_id(&self.connection);
                        self.vector = actualizar_tabla(self.vector.clone(), &self.connection);
                        self.int_option += 1;
                        let alternatives = list_id(&self.connection);
                        if let Some(updated_index) = alternatives.last() { self.id = *updated_index as i32 }
                    }
                    ui.label("");
                    if ui.button("Modificar").clicked()&&self.id!=0
                        && validation(
                            &self.nombre,
                            &self.apellidos,
                            &self.nacionalidad,
                            &self.fecha,
                        )
                    {
                        let text = "¿Quiéres modificar el usuario con id ".to_owned()
                            + &self.id.to_string()
                            + "?";
                        let yes = MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Modificar usuario")
                            .set_text(&text)
                            .show_confirm()
                            .unwrap();
                        if yes {
                            let _usuario = crear_usuario(
                                self.id,
                                self.apellidos.clone(),
                                self.nombre.clone(),
                                self.genero.to_string(),
                                self.nacionalidad.clone(),
                                self.fecha.clone(),
                            );
                            _usuario.update_db(&self.connection).unwrap();
                            self.contador = count_id(&self.connection);
                            self.vector = actualizar_tabla(self.vector.clone(), &self.connection);
                        }
                    }
                    ui.label("");
                    if ui.button("Borrar").clicked()&&self.id!=0 {
                        let text = "¿Quiéres borrar el usuario con id ".to_owned()
                            + &self.id.to_string()
                            + "?";
                        let yes = MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Borrar usuario")
                            .set_text(&text)
                            .show_confirm()
                            .unwrap();
                        if yes {
                            let _usuario = crear_usuario(
                                self.id,
                                self.apellidos.clone(),
                                self.nombre.clone(),
                                self.genero.to_string(),
                                self.nacionalidad.clone(),
                                self.fecha.clone(),
                            );
                            let alternatives = list_id(&self.connection);
                            let index = alternatives
                                .iter()
                                .position(|&r| r == self.id as i64)
                                .unwrap();
                            _usuario.delete_from_db(&self.connection).unwrap();
                            if index!=0{
                                if let Some(updated_index) = alternatives.get(index - 1) { self.id = *updated_index as i32 }
                            }else{
                                match alternatives.get(index + 1) {
                                    Some(updated_index) => self.id = *updated_index as i32,
                                    None => self.id=0,
                                }
                            }
                            self.contador = count_id(&self.connection);
                            self.vector = actualizar_tabla(self.vector.clone(), &self.connection);
                        }
                    }
                    ui.label("");
                    if ui.button("Limpiar").clicked() {
                        self.nombre = "".to_string();
                        self.apellidos = "".to_string();
                        self.genero = Genero::Masculino;
                        self.nacionalidad = "".to_string();
                        self.fecha = "".to_string();
                    }
                });
                if self.contador != 0 {
                    let alternatives = list_id(&self.connection);
                    ui.label(format!("ID seleccionada: {}", self.id));
                    egui::ComboBox::from_id_source(1)
                        .selected_text(self.id.to_string())
                        .show_ui(ui, |ui| {
                            for option in alternatives {
                                ui.selectable_value(
                                    &mut self.id,
                                    option as i32,
                                    (option as i32).to_string(),
                                );
                            }
                        });
                }
            });

        CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::initial(50.0).at_least(50.0).at_most(50.0))
                .column(Column::initial(160.0).at_least(50.0).at_most(160.))
                .column(Column::initial(160.0).at_least(50.0).at_most(160.))
                .column(Column::initial(160.0).at_least(50.0).at_most(160.))
                .column(Column::initial(160.0).at_least(50.0).at_most(160.))
                .column(Column::initial(160.0).at_least(50.0).at_most(160.))
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .resizable(false)
                .header(20.0, |mut header| {
                    for text in [
                        "Id",
                        "Apellidos",
                        "Nombre",
                        "Sexo",
                        "Nacionalidad",
                        "Fecha Nacimiento",
                    ] {
                        header.col(|ui| {
                            ui.heading(text);
                        });
                    }
                })
                .body(|body| {
                    body.rows(
                        25.0,
                        self.contador.try_into().unwrap(),
                        |row_idx, mut row| {
                            row.col(|ui| {
                                ui.label(&self.vector[row_idx][0]);
                            });
                            row.col(|ui| {
                                ui.label(&self.vector[row_idx][1]);
                            });
                            row.col(|ui| {
                                ui.label(&self.vector[row_idx][2]);
                            });
                            row.col(|ui| {
                                ui.label(&self.vector[row_idx][3]);
                            });
                            row.col(|ui| {
                                ui.label(&self.vector[row_idx][4]);
                            });
                            row.col(|ui| {
                                ui.label(&self.vector[row_idx][5]);
                            });
                        },
                    )
                })
        });
    }
}

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(Vec2::new(1170., 535.)),
        min_window_size: Some(Vec2::new(1170., 535.)),
        resizable: false,
        default_theme: Theme::Light,
        ..Default::default()
    };
    run_native("Cyberdyne", options, Box::new(|_| Box::<Test>::default()));
}

#[derive(Debug)]
pub struct Usuario {
    pub(crate) id: i32,
    pub(crate) apellidos: String,
    pub(crate) nombre: String,
    pub(crate) sexo: String,
    pub(crate) nacionalidad: String,
    pub(crate) nacimiento: String,
}

impl Usuario {
    pub fn insert_into_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        let mut stmt = conn.prepare("INSERT INTO usuarios (apellidos, nombre, sexo, nacionalidad, f_nacimiento) VALUES (?, ?, ?, ?, ?)").unwrap();
        stmt.bind((1, self.apellidos.as_str())).unwrap();
        stmt.bind((2, self.nombre.as_str())).unwrap();
        stmt.bind((3, self.sexo.as_str())).unwrap();
        stmt.bind((4, self.nacionalidad.as_str())).unwrap();
        stmt.bind((5, self.nacimiento.as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }

    pub fn delete_from_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        let mut stmt = conn.prepare("DELETE FROM usuarios WHERE id = ?").unwrap();
        stmt.bind((1, self.id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }

    pub fn update_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        let mut stmt = conn.prepare("UPDATE usuarios SET apellidos = ?, nombre = ?, sexo = ?, nacionalidad = ?, f_nacimiento = ? WHERE id = ?").unwrap();
        stmt.bind((1, self.apellidos.as_str())).unwrap();
        stmt.bind((2, self.nombre.as_str())).unwrap();
        stmt.bind((3, self.sexo.as_str())).unwrap();
        stmt.bind((4, self.nacionalidad.as_str())).unwrap();
        stmt.bind((5, self.nacimiento.as_str())).unwrap();
        stmt.bind((6, self.id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }
}

pub fn get_connection() -> sqlite::Connection {
    sqlite::open(":memory:").unwrap()
}

pub fn create_table(conn: &sqlite::Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY,
            apellidos VARCHAR NOT NULL,
            nombre VARCHAR NOT NULL,
            sexo VARCHAR NOT NULL,
            nacionalidad VARCHAR NOT NULL,
            f_nacimiento DATE NOT NULL)",
    )
    .unwrap();
}

pub fn select_user(conn: &sqlite::Connection) -> Vec<Usuario> {
    let select = "SELECT * FROM usuarios";
    let mut statement = conn.prepare(select).unwrap();
    let mut vector_usuario = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let user = Usuario {
            id: statement.read::<i64, _>("id").unwrap() as i32,
            apellidos: statement.read::<String, _>("apellidos").unwrap(),
            nombre: statement.read::<String, _>("nombre").unwrap(),
            sexo: statement.read::<String, _>("sexo").unwrap(),
            nacionalidad: statement.read::<String, _>("nacionalidad").unwrap(),
            nacimiento: statement.read::<String, _>("f_nacimiento").unwrap(),
        };
        vector_usuario.push(user);
    }
    vector_usuario
}

pub fn count_id(conn: &sqlite::Connection) -> i32 {
    let select = "SELECT * FROM usuarios";
    let mut statement = conn.prepare(select).unwrap();
    // let mut _id_max: i64 = 0;
    let mut contador = 0;
    while let Ok(State::Row) = statement.next() {
        // _id_max = statement.read::<i64, _>("id").unwrap();
        contador += 1
    }
    // _id_max as i32
    contador
}

pub fn list_id(conn: &sqlite::Connection) -> Vec<i64> {
    let select = "SELECT * FROM usuarios";
    let mut statement = conn.prepare(select).unwrap();
    let mut vector_id = Vec::new();
    while let Ok(State::Row) = statement.next() {
        vector_id.push(statement.read::<i64, _>("id").unwrap());
    }
    vector_id
}

pub fn validation(nombre: &str, apellidos: &str, nacionalidad: &str, fecha: &str) -> bool {
    if nombre.is_empty() || apellidos.is_empty() || nacionalidad.is_empty() || fecha.is_empty() {
        let message = "Algunos campos están vacíos".to_string();
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Error")
            .set_text(&message)
            .show_alert()
            .unwrap();
        return false;
    } else if nacionalidad.len() != 3 {
        let message = "Nacionalidad tiene que tener 3 caracteres".to_string();
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Error")
            .set_text(&message)
            .show_alert()
            .unwrap();
        return false;
    } else if !fecha.contains('-') {
        let message = "Nacimiento tiene que tener el formato dd-mm-yyyy".to_string();
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Error")
            .set_text(&message)
            .show_alert()
            .unwrap();
        return false;
    }
    true
}

pub fn crear_usuario(
    id: i32,
    apellidos: String,
    nombre: String,
    genero: String,
    nacionalidad: String,
    fecha: String,
) -> Usuario {
    Usuario {
        id: id.to_owned(),
        apellidos,
        nombre,
        sexo: genero,
        nacionalidad,
        nacimiento: fecha,
    }
}

pub fn actualizar_tabla(
    mut vector: Vec<[String; 6]>,
    conexion: &sqlite::Connection,
) -> Vec<[String; 6]> {
    let vector_usuario = select_user(conexion);
    vector.clear();
    for usuario in vector_usuario {
        vector.push([
            usuario.id.to_string(),
            usuario.apellidos,
            usuario.nombre,
            usuario.sexo,
            usuario.nacionalidad,
            usuario.nacimiento,
        ]);
    }
    vector
}
