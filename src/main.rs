use eframe::{NativeOptions, App, egui::{CentralPanel, Direction, Layout}};
use egui_extras::{TableBuilder, Column};
use egui::{Vec2,SidePanel};
use eframe::run_native;
use native_dialog::{MessageDialog, MessageType};
use std::fmt;
use prettytable::{row, Table};
use sqlite::State;
use sqlite::{Error as SqliteError};

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
    test_data: Vec<[String; 6]>,
    nombre: String,
    apellidos: String,
    genero: Genero,
    nacionalidad: String,
    fecha: String,
    _connection: sqlite::Connection,
    is_table_created: bool,
    id: i32,
    contador: i32,
}

impl Default for Test {
    fn default() -> Self {
        let test_data = Vec::with_capacity(1000);
        Self { test_data, nombre: "".to_string(), apellidos: "".to_string(), genero: Genero::Masculino, nacionalidad: "".to_string(), fecha: "".to_string(), _connection: get_connection(), is_table_created: false, id: 1, contador: 0 }
    }
}

impl App for Test {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

        if self.is_table_created.eq(&false) {
            create_table(&self._connection);
            self.is_table_created = true;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menú", |ui| {
                    if ui.button("Cerrar").clicked() {
                        _frame.close();
                    }
                });
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
                egui::ComboBox::from_label(format!(""))
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
                    if ui.button("Alta").clicked() {
                        if self.nombre.is_empty()
                            || self.apellidos.is_empty()
                            || self.nacionalidad.is_empty()
                            || self.fecha.is_empty()
                        {
                            let message = format!("Algunos campos están vacíos");
                            MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("Error")
                                .set_text(&*message)
                                .show_alert()
                                .unwrap();
                        } else if self.nacionalidad.len() != 3 {
                            let message = format!("Nacionalidad tiene que tener 3 caracteres");
                            MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("Error")
                                .set_text(&*message)
                                .show_alert()
                                .unwrap();
                        } else if !self.fecha.contains("-") {
                            let message = format!("Nacimiento tiene que tener el formato dd-mm-yyyy");
                            MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("Error")
                                .set_text(&*message)
                                .show_alert()
                                .unwrap();
                        } else {
                            let _usuario = Usuario {
                                _id: self.id.to_owned(),
                                _apellidos: self.apellidos.to_owned(),
                                _nombre: self.nombre.to_owned(),
                                _sexo: self.genero.to_string().to_owned(),
                                _nacionalidad: self.nacionalidad.to_owned(),
                                _nacimiento: self.fecha.to_owned(),
                            };
                            _usuario.insert_into_db(&self._connection).unwrap();
                            self.contador = count_id(&self._connection);
                            let vector_usuario = select_user(&self._connection);
                            self.test_data.clear();
                            for usuario in vector_usuario {
                                self.test_data.push([
                                    usuario._id.to_string(),
                                    usuario._apellidos,
                                    usuario._nombre,
                                    usuario._sexo,
                                    usuario._nacionalidad,
                                    usuario._nacimiento,
                                ]);
                            }
                        }
                    }
                    ui.label("");
                    if ui.button("Modificar").clicked() {
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
                            let _usuario = Usuario {
                                _id: self.id.to_owned(),
                                _apellidos: self.apellidos.to_owned(),
                                _nombre: self.nombre.to_owned(),
                                _sexo: self.genero.to_string().to_owned(),
                                _nacionalidad: self.nacionalidad.to_owned(),
                                _nacimiento: self.fecha.to_owned(),
                            };
                            _usuario.update_db(&self._connection).unwrap();
                            self.contador = count_id(&self._connection);
                            let vector_usuario = select_user(&self._connection);
                            self.test_data.clear();
                            for usuario in vector_usuario {
                                self.test_data.push([
                                    usuario._id.to_string(),
                                    usuario._apellidos,
                                    usuario._nombre,
                                    usuario._sexo,
                                    usuario._nacionalidad,
                                    usuario._nacimiento,
                                ]);
                            }
                        }
                    }
                    ui.label("");
                    if ui.button("Borrar").clicked() {
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
                            let _usuario = Usuario {
                                _id: self.id.to_owned(),
                                _apellidos: self.apellidos.to_owned(),
                                _nombre: self.nombre.to_owned(),
                                _sexo: self.genero.to_string().to_owned(),
                                _nacionalidad: self.nacionalidad.to_owned(),
                                _nacimiento: self.fecha.to_owned(),
                            };
                            _usuario.delete_from_db(&self._connection).unwrap();
                            // let id_vector = self.id-1;
                            // self.test_data.remove(id_vector.try_into().unwrap());
                            // self.contador = count_id(&self._connection);
                            self.id -= 1;
                            self.contador = count_id(&self._connection);
                            let vector_usuario = select_user(&self._connection);
                            self.test_data.clear();
                            for usuario in vector_usuario {
                                self.test_data.push([
                                    usuario._id.to_string(),
                                    usuario._apellidos,
                                    usuario._nombre,
                                    usuario._sexo,
                                    usuario._nacionalidad,
                                    usuario._nacimiento,
                                ]);
                            }
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
                    ui.label("Seleccione el ID (Modificar/Borrar):");
                    ui.add(egui::Slider::new(&mut self.id, 1..=self.contador).text(""));
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
                    for text in ["Id", "Apellidos", "Nombre", "Sexo", "Nacionalidad", "Fecha Nacimiento"] {
                        header.col(|ui| {
                            ui.heading(text);
                        });
                    }
                })
                .body(|body| {
                    body.rows(25.0, self.contador.try_into().unwrap(), |row_idx, mut row| {
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][0]);
                        });
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][1]);
                        });
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][2]);
                        });
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][3]);
                        });
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][4]);
                        });
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][5]);
                        });
                    })
                })
        });
    }
}

fn main(){
    let options = NativeOptions {
        initial_window_size: Some(Vec2::new(1170., 535.)),
        min_window_size: Some(Vec2::new(1170., 535.)),
        resizable: false,
        ..Default::default()
    };
    run_native(
        "Test",
        options,
        Box::new(|_| Box::new(Test::default()))
    );
}

#[derive(Debug)]
pub struct Usuario {
    pub(crate) _id: i32,
    pub(crate) _apellidos: String,
    pub(crate) _nombre: String,
    pub(crate) _sexo: String,
    pub(crate) _nacionalidad: String,
    pub(crate) _nacimiento: String,
}

impl Usuario {
    pub fn insert_into_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        // Conectarse a la base de datos y preparar la instrucción SQL para insertar un usuario
        //let conn = sqlite::open(":memory:").unwrap();
        let mut stmt = conn.prepare("INSERT INTO usuarios (apellidos, nombre, sexo, nacionalidad, f_nacimiento) VALUES (?, ?, ?, ?, ?)").unwrap();
        stmt.bind((1, self._apellidos.as_str())).unwrap();
        stmt.bind((2, self._nombre.as_str())).unwrap();
        stmt.bind((3, self._sexo.as_str())).unwrap();
        stmt.bind((4, self._nacionalidad.as_str())).unwrap();
        stmt.bind((5, self._nacimiento.as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }

    pub fn delete_from_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        // Conectarse a la base de datos y preparar la instrucción SQL para insertar un usuario
        //let conn = sqlite::open(":memory:").unwrap();
        let mut stmt = conn.prepare("DELETE FROM usuarios WHERE id = ?").unwrap();
        stmt.bind((1, self._id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }

    pub fn update_db(&self, conn: &sqlite::Connection) -> Result<(), SqliteError> {
        // Conectarse a la base de datos y preparar la instrucción SQL para insertar un usuario
        //let conn = sqlite::open(":memory:").unwrap();
        let mut stmt = conn.prepare("UPDATE usuarios SET apellidos = ?, nombre = ?, sexo = ?, nacionalidad = ?, f_nacimiento = ? WHERE id = ?").unwrap();
        stmt.bind((1, self._apellidos.as_str())).unwrap();
        stmt.bind((2, self._nombre.as_str())).unwrap();
        stmt.bind((3, self._sexo.as_str())).unwrap();
        stmt.bind((4, self._nacionalidad.as_str())).unwrap();
        stmt.bind((5, self._nacimiento.as_str())).unwrap();
        stmt.bind((6, self._id.to_string().as_str())).unwrap();
        stmt.next().unwrap();
        Ok(())
    }
}

pub fn get_connection() -> sqlite::Connection {
    let connection = sqlite::open(":memory:").unwrap();
    connection
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

pub fn select_user(conn: &sqlite::Connection) -> Vec<Usuario>{
    // Create the table
    let mut table = Table::new();
    table.add_row(row![
        "ID",
        "Apellidos",
        "Nombre",
        "Sexo",
        "Nacionalidad",
        "Fecha nacimiento"
    ]);

    let select = "SELECT * FROM usuarios";
    let mut statement = conn.prepare(select).unwrap();
    // println!("Usuarios encontrados:");
    let mut contador = 0;
    let mut vector_usuario = Vec::new();
    while let Ok(State::Row) = statement.next() {
        /*println!("* Usuario con id = {}", statement.read::<i64, _>("id").unwrap());
        println!("- nombre = {}", statement.read::<String, _>("nombre").unwrap());
        println!("- apellidos = {}", statement.read::<String,_>("apellidos").unwrap());
        println!("- sexo = {}", statement.read::<String, _>("sexo").unwrap());
        println!("- nacionalidad = {}", statement.read::<String, _>("nacionalidad").unwrap());
        println!("- nacimiento = {}", statement.read::<String, _>("f_nacimiento").unwrap());*/
        table.add_row(row![
            statement.read::<i64, _>("id").unwrap(),
            statement.read::<String, _>("apellidos").unwrap(),
            statement.read::<String, _>("nombre").unwrap(),
            statement.read::<String, _>("sexo").unwrap(),
            statement.read::<String, _>("nacionalidad").unwrap(),
            statement.read::<String, _>("f_nacimiento").unwrap()
        ]);

        let user = Usuario{
            _id: statement.read::<i64, _>("id").unwrap() as i32,
            _apellidos: statement.read::<String, _>("apellidos").unwrap(),
            _nombre: statement.read::<String, _>("nombre").unwrap(),
            _sexo: statement.read::<String, _>("sexo").unwrap(),
            _nacionalidad: statement.read::<String, _>("nacionalidad").unwrap(),
            _nacimiento: statement.read::<String, _>("f_nacimiento").unwrap(),
        };

        vector_usuario.push(user);

        contador = contador + 1;
    }
    // table.printstd();

    // if contador == 1 {
    //     println!("Se ha encontrado sólo {} usuario", contador);
    // } else {
    //     println!("Se han encontrado {} usuarios", contador);
    // }
    vector_usuario
}

pub fn count_id(conn: &sqlite::Connection) -> i32 {
    let select = "SELECT * FROM usuarios";
    let mut statement = conn.prepare(select).unwrap();
    let mut _id_max:i64 = 0;
    let mut contador = 0;
    while let Ok(State::Row) = statement.next() {
        _id_max = statement.read::<i64, _>("id").unwrap();
        contador += 1
    }
    // _id_max as i32
    contador
}