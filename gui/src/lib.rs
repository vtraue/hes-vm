use data::SectionType;
use eframe::{
    egui::{
        self,
        ahash::{HashSet, HashSetExt},
        Color32, ComboBox, Label, RichText, ScrollArea, Sense, TextStyle, TextWrapMode, Ui, Vec2,
        Widget,
    },
    epaint::ColorMode,
    App,
};
use egui_dock::{AllowedSplits, DockArea, DockState, NodeIndex, Style, SurfaceIndex, TabViewer};
use std::fs;
use vm::{
    bytecode_info::{BytecodeInfo, Function},
    reader::{self, Position, Reader},
};

pub mod data;

struct MyContext {
    pub title: String,
    pub style: Option<Style>,

    picked_path: Option<String>,
    bytecode: Vec<u8>,
    bytecode_info: BytecodeInfo,
    bytecode_option: BytecodeDisplayOptions,
    frame_data: BetweenFrameData,

    open_tabs: HashSet<String>,

    show_close_buttons: bool,
    show_add_buttons: bool,
    draggable_tabs: bool,
    show_tab_name_on_hover: bool,
    allowed_splits: AllowedSplits,
    show_leaf_close_all: bool,
    show_leaf_collapse: bool,
    show_secondary_button_hint: bool,
    secondary_button_on_modifier: bool,
    secondary_button_context_menu: bool,
}
pub struct HesApp {
    tree: DockState<String>,
    context: MyContext,
    show_own_code: bool,
    show_wat: bool,
    show_wasm: bool,
}

impl TabViewer for MyContext {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "HES-VM" => self.simple_demo(ui),
            "Style Editor" => self.style_editor(ui),
            "Bytecode" => self.bytecode(ui),
            "Instructions" => self.instructions(ui),
            "Bytecode Infos" => self.bytecode_information(ui),
            _ => {
                ui.label(tab.as_str());
            }
        }
    }

    fn context_menu(
        &mut self,
        ui: &mut egui::Ui,
        tab: &mut Self::Tab,
        _surface: SurfaceIndex,
        _node: NodeIndex,
    ) {
        match tab.as_str() {
            "HES-VM" => self.simple_demo_menu(ui),
            _ => {
                ui.label(tab.to_string());
                ui.label("This is a context menu");
            }
        }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        ["Inspector", "Style Editor"].contains(&tab.as_str())
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.open_tabs.remove(tab);
        true
    }
}

impl MyContext {
    fn bytecode(&mut self, ui: &mut egui::Ui) {
        draw_bytecode(
            ui,
            &self.bytecode,
            &self.bytecode_option,
            &mut self.frame_data,
        );
    }
    fn bytecode_information(&mut self, ui: &mut egui::Ui) {
        draw_bytecode_info(ui, &self.bytecode_info, &mut self.frame_data);
    }
    fn instructions(&mut self, ui: &mut egui::Ui) {
        draw_code_text(ui, &self.bytecode_info, &mut self.frame_data);
    }
    fn simple_demo_menu(&mut self, ui: &mut Ui) {
        ui.label("Egui widget example");
        ui.menu_button("Sub menu", |ui| {
            ui.label("hello :)");
        });
    }

    fn simple_demo(&mut self, ui: &mut Ui) {
        ui.heading("HES-VM");

        ui.horizontal(|ui| {
            ui.label("Hier wird (bald) das eigene Programm ausgeführt");
        });
    }

    fn style_editor(&mut self, ui: &mut Ui) {
        ui.heading("Style Editor");
        ui.collapsing("DockArea Options", |ui| {
            ui.checkbox(&mut self.show_close_buttons, "Show close buttons");
            ui.checkbox(&mut self.show_add_buttons, "Show add buttons");
            ui.checkbox(&mut self.draggable_tabs, "Draggable tabs");
            ui.checkbox(&mut self.show_tab_name_on_hover, "Show tab name on hover");
            ui.checkbox(
                &mut self.show_leaf_close_all,
                "Show close all button on tab bars",
            );
            ui.checkbox(
                &mut self.show_leaf_collapse,
                "Show collapse button on tab bar",
            );
            ui.checkbox(
                &mut self.secondary_button_on_modifier,
                "Enable secondary buttons when modifiers (Shit by default) are pressed",
            );
            ui.checkbox(
                &mut self.secondary_button_context_menu,
                "Enable secondary buttons in right-click context menus",
            );
            ui.checkbox(
                &mut self.show_secondary_button_hint,
                "Show tooltip hints for secondary buttons",
            );

            ComboBox::new("cbox:allowed_splits", "Split direction(s)")
                .selected_text(format!("{:?}", self.allowed_splits))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.allowed_splits, AllowedSplits::All, "All");
                    ui.selectable_value(
                        &mut self.allowed_splits,
                        AllowedSplits::LeftRightOnly,
                        "LeftRightOnly",
                    );
                    ui.selectable_value(
                        &mut self.allowed_splits,
                        AllowedSplits::TopBottomOnly,
                        "TopBottomOnly",
                    );
                    ui.selectable_value(&mut self.allowed_splits, AllowedSplits::None, "None");
                });
        });

        // --snip--
    }
}

impl<'src, 'b> Default for HesApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["HES-VM".to_owned(), "Style Editor".to_owned()]);
        "Undock".clone_into(&mut tree.translations.tab_context_menu.eject_button);
        // modify tree before constructing the dock:
        let [_, b] = tree.main_surface_mut().split_left(
            NodeIndex::root(),
            0.2,
            vec!["Bytecode Infos".to_owned()],
        );
        let [_, _] = tree.main_surface_mut().split_left(
            NodeIndex::root(),
            0.2,
            vec!["Instructions".to_owned()],
        );
        let [_, _] = tree
            .main_surface_mut()
            .split_below(b, 0.5, vec!["Bytecode".to_owned()]);

        let mut open_tabs = HashSet::new();

        for node in tree[SurfaceIndex::main()].iter() {
            if let Some(tabs) = node.tabs() {
                for tab in tabs {
                    open_tabs.insert(tab.clone());
                }
            }
        }

        let context = MyContext {
            title: "HES-VM".to_string(),
            style: None,
            picked_path: None,
            bytecode_info: BytecodeInfo::default(),
            bytecode: vec![],
            bytecode_option: BytecodeDisplayOptions::default(),
            frame_data: Default::default(),
            open_tabs,
            show_close_buttons: true,
            show_add_buttons: true,
            draggable_tabs: true,
            show_tab_name_on_hover: true,
            allowed_splits: AllowedSplits::default(),
            show_leaf_close_all: true,
            show_leaf_collapse: true,
            show_secondary_button_hint: true,
            secondary_button_on_modifier: true,
            secondary_button_context_menu: true,
        };

        Self {
            tree,
            context,
            show_own_code: true,
            show_wat: true,
            show_wasm: true,
        }
    }
}

impl<'src, 'b> HesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, path: Option<&str>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}
        let mut app = Self {
            ..Default::default()
        };

        match path {
            Some(path) => {
                app.context.bytecode = fs::read(path).unwrap();
            }
            None => (),
        }

        let reader = Reader::new(&app.context.bytecode, 0);

        match BytecodeInfo::from_reader(&reader) {
            Ok(info) => app.context.bytecode_info = info,
            Err(_) => todo!(),
        };

        app
    }
}

impl<'src> App for HesApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Open file...").clicked() {
                            // TODO: (viv): BytecodeHighlight muss resettet werden!!!
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.context.picked_path = Some(path.display().to_string());
                                self.context.bytecode = fs::read(path).unwrap();
                                let reader = Reader::new(&self.context.bytecode, 0);
                                self.context.bytecode_info =
                                    BytecodeInfo::from_reader(&reader).unwrap();
                            }
                            ui.close_menu();
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_own_code, "Your Code");
                    ui.checkbox(&mut self.show_wat, "WAT Textformat");
                    ui.checkbox(&mut self.show_wasm, "WASM Bytecode");
                });

                egui::widgets::global_theme_preference_buttons(ui);
            });
            ui.vertical_centered(|ui| {
                ui.heading("HES-VM");
            });
        });

        DockArea::new(&mut self.tree)
            .show_close_buttons(self.context.show_close_buttons)
            .show_add_buttons(self.context.show_add_buttons)
            .draggable_tabs(self.context.draggable_tabs)
            .show_tab_name_on_hover(self.context.show_tab_name_on_hover)
            .allowed_splits(self.context.allowed_splits)
            .show_leaf_close_all_buttons(self.context.show_leaf_close_all)
            .show_leaf_collapse_buttons(self.context.show_leaf_collapse)
            .show_secondary_button_hint(self.context.show_secondary_button_hint)
            .secondary_button_on_modifier(self.context.secondary_button_on_modifier)
            .secondary_button_context_menu(self.context.secondary_button_context_menu)
            .show(ctx, &mut self.context);

        // egui::TopBottomPanel::bottom("bottom_panel")
        //     .resizable(false)
        //     .min_height(0.0)
        //     .show(ctx, |ui| {
        //         ui.separator();
        //
        //         ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //             powered_by_egui_and_eframe(ui);
        //             egui::warn_if_debug_build(ui);
        //         });
        //     });
        // egui::SidePanel::left("left_panel")
        //     .resizable(true)
        //     .default_width(450.0)
        //     .width_range(20.0..)
        //     .show_animated(ctx, self.show_own_code, |ui| {
        //         ui.vertical_centered(|ui| {
        //             ui.heading("Your code");
        //         });
        //         egui::ScrollArea::vertical().show(ui, |ui| {
        //             ui.label(
        //                 "Irgendwann steht hier der Sourcecode der compilierten (eigenen) Sprache.",
        //             )
        //         })
        //     });
        //
        // egui::SidePanel::right("right_panel")
        //     .resizable(true)
        //     .default_width(450.0)
        //     .width_range(20.0..)
        //     .show_animated(ctx, self.show_wasm, |ui| {
        //         ui.vertical_centered(|ui| {
        //             ui.heading("Your generated Bytecode (WASM)");
        //         });
        //         draw_bytecode(
        //             ui,
        //             &self.bytecode,
        //             &self.bytecode_option,
        //             &mut self.frame_data,
        //         );
        //     });
        //
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     if self.show_wat {
        //         ui.vertical_centered(|ui| {
        //             ui.heading("Your Code as WAT (Wasm Text Format)");
        //         });
        //         draw_code_text(ui, &self.code_text);
        //     }
        // });
    }
}

fn draw_bytecode_info(
    ui: &mut egui::Ui,
    bytecode_info: &BytecodeInfo,
    frame_data: &mut BetweenFrameData,
) {
    // ausklappbar:
    // - Imports
    // - Globals
    // - Types
    // - Funktionen mit Parametern
    // - Start Section
    // - Speicher, Stack
    ui.collapsing("ImportSection", |ui| {
        draw_imports(ui, bytecode_info, frame_data);
    });
    ui.collapsing("GlobalSection", |ui| {
        draw_globals(ui, bytecode_info, frame_data);
    });
    ui.collapsing("TypeSection", |ui| {
        draw_types(ui, bytecode_info, frame_data);
    });
    ui.collapsing("FunctionSection", |ui| {
        let text = RichText::new(
            "Auflistung aller Funktionen mit ihren dazugehöhrigen Typen (aus der TypeSection)",
        );
        let highlight = BytecodeHighlight {
            position_bytecode: bytecode_info.function_section.as_ref().unwrap().1,
            selected_token_wat: None,
            position_info: Some(PositionInfo {
                section: SectionType::Function,
                idx: 0,
            }),
            highlight_type: HighlightType::Border,
            highlight_color: Color32::RED,
        };
        draw_highlightable_info_label(ui, text, highlight, frame_data);
        draw_function_headers(ui, bytecode_info, frame_data);
    });
    ui.collapsing("StartSection", |ui| {
        draw_start_section(ui, bytecode_info, frame_data);
    });
}

fn draw_highlightable_info_label(
    ui: &mut Ui,
    text: RichText,
    highlight: BytecodeHighlight,
    frame_data: &mut BetweenFrameData,
) {
    let mut text = text;
    match &highlight.position_info {
        Some(pos) => match frame_data.should_highlight_info(pos) {
            Some((color, hl_type)) => match hl_type {
                HighlightType::Background => text = text.background_color(color),
                HighlightType::Border => text = text.color(color).underline(),
                HighlightType::Bold => text = text.color(color).strong(),
            },
            None => (),
        },
        None => (),
    };

    let response = Label::new(text).ui(ui);

    if response.clicked() {
        frame_data.toggle_bytecode_highlight(highlight);
    }
}

fn draw_highlightable_wat_label(
    ui: &mut Ui,
    text: RichText,
    highlight: BytecodeHighlight,
    frame_data: &mut BetweenFrameData,
) {
    let mut text = text;
    match &highlight.selected_token_wat {
        Some(pos) => match frame_data.should_highlight_wat(pos) {
            Some((color, hl_type)) => match hl_type {
                HighlightType::Background => text = text.background_color(color),
                HighlightType::Border => text = text.color(color).underline(),
                HighlightType::Bold => text = text.color(color).strong(),
            },
            None => (),
        },
        None => (),
    };

    let response = Label::new(text).ui(ui);

    if response.clicked() {
        frame_data.toggle_bytecode_highlight(highlight);
    }
}

fn draw_types(ui: &mut Ui, bytecode_info: &BytecodeInfo, frame_data: &mut BetweenFrameData) {
    let types = bytecode_info.type_section.as_ref().unwrap();
    for (i, t) in types.0.iter().enumerate() {
        let pos_func_type = PositionInfo {
            section: SectionType::Type,
            idx: i,
        };
        let highlight = BytecodeHighlight {
            position_bytecode: t.1,
            selected_token_wat: None,
            position_info: Some(pos_func_type),
            highlight_type: HighlightType::Background,
            highlight_color: Color32::ORANGE,
        };

        draw_highlightable_info_label(
            ui,
            RichText::new(format!("Funktionstype {}", i)),
            highlight,
            frame_data,
        );

        ui.indent("Functiontype", |ui| {
            // TODO: (viv): Das ist noch nicht ganz richtig, hier sollten nur die Parameter gehighlightet
            // werden, nicht der ganze Funktionstyp
            // vorher die Position des letzten Parameter nehmen und eine neue Position
            // zusammenbasteln?
            let pos_info_params = PositionInfo {
                section: SectionType::Type,
                idx: i,
            };
            let highlight = BytecodeHighlight {
                position_bytecode: t.1,
                selected_token_wat: None,
                position_info: Some(pos_info_params),
                highlight_type: HighlightType::Border,
                highlight_color: Color32::GREEN,
            };

            draw_highlightable_info_label(
                ui,
                RichText::new(format!("Params: ",)),
                highlight,
                frame_data,
            );

            ui.indent("Params", |ui| {
                for (j, param) in t.0.params.iter().enumerate() {
                    let pos_info = PositionInfo {
                        section: SectionType::Type,
                        idx: j,
                    };
                    let highlight = BytecodeHighlight {
                        position_bytecode: param.1,
                        selected_token_wat: None,
                        position_info: Some(pos_info),
                        highlight_type: HighlightType::Bold,
                        highlight_color: Color32::GREEN,
                    };

                    draw_highlightable_info_label(
                        ui,
                        RichText::new(format!("{}", param.0)),
                        highlight,
                        frame_data,
                    );
                }
            });
        });
    }
}

fn draw_function_headers(
    ui: &mut Ui,
    bytecode_info: &BytecodeInfo,
    frame_data: &mut BetweenFrameData,
) {
    let functions = bytecode_info.function_section.as_ref().unwrap();
    for (i, function) in functions.0.iter().enumerate() {
        let pos = PositionInfo {
            section: SectionType::Function,
            idx: i,
        };
        let highlight = BytecodeHighlight {
            position_bytecode: function.1,
            selected_token_wat: None,
            position_info: Some(pos),
            highlight_type: HighlightType::Background,
            highlight_color: Color32::DARK_BLUE,
        };
        draw_highlightable_info_label(
            ui,
            RichText::new(format!("Function {}", i)),
            highlight.clone(),
            frame_data,
        );
        ui.indent("Function", |ui| {
            draw_highlightable_info_label(
                ui,
                RichText::new(format!("Funktionstyp: {}", function.0)),
                highlight,
                frame_data,
            );
        });
    }
}

fn draw_start_section(
    ui: &mut Ui,
    bytecode_info: &BytecodeInfo,
    frame_data: &mut BetweenFrameData,
) {
    let start = bytecode_info.start_section.as_ref().unwrap();
    let pos = PositionInfo {
        section: SectionType::Start,
        idx: 0,
    };
    let highlight = BytecodeHighlight {
        position_bytecode: start.1,
        selected_token_wat: None,
        position_info: Some(pos),
        highlight_type: HighlightType::Background,
        highlight_color: Color32::DARK_RED,
    };
    draw_highlightable_info_label(
        ui,
        RichText::new(format!("FuncId: {}", start.0)),
        highlight,
        frame_data,
    );
}

fn draw_imports(
    ui: &mut egui::Ui,
    bytecode_info: &BytecodeInfo,
    frame_data: &mut BetweenFrameData,
) {
    let import_section = bytecode_info.import_section.as_ref().unwrap();
    for (i, import) in import_section.0.iter().enumerate() {
        let pos = PositionInfo {
            section: SectionType::Import,
            idx: i,
        };
        let highlight = BytecodeHighlight {
            position_bytecode: import.1,
            selected_token_wat: None,
            position_info: Some(pos),
            highlight_type: HighlightType::Background,
            highlight_color: Color32::MAGENTA,
        };
        ui.horizontal_wrapped(|ui| {
            let text1 = RichText::new(format!("Import {}: ", i)).strong();
            let text2 = RichText::new(format!("{}", import.0));
            draw_highlightable_info_label(ui, text1, highlight.clone(), frame_data);
            draw_highlightable_info_label(ui, text2, highlight, frame_data);
        });
    }
}

fn draw_globals(
    ui: &mut egui::Ui,
    bytecode_info: &BytecodeInfo,
    frame_data: &mut BetweenFrameData,
) {
    let global_section = bytecode_info.global_section.as_ref().unwrap();
    for (i, global) in global_section.0.iter().enumerate() {
        let pos = PositionInfo {
            section: SectionType::Global,
            idx: i,
        };
        let highlight = BytecodeHighlight {
            position_bytecode: global.1,
            selected_token_wat: None,
            position_info: Some(pos),
            highlight_type: HighlightType::Background,
            highlight_color: Color32::KHAKI,
        };
        draw_highlightable_info_label(
            ui,
            RichText::new(format!("Global {}", i)),
            highlight.clone(),
            frame_data,
        );
        ui.indent("Global", |ui| {
            draw_highlightable_info_label(
                ui,
                RichText::new(format!("{}", global.0)),
                highlight,
                frame_data,
            );
        });
    }
}

fn draw_code_text(
    ui: &mut egui::Ui,
    bytecode_info: &BytecodeInfo,
    frame_data: &mut BetweenFrameData,
) {
    let code = bytecode_info.code_section.as_ref().unwrap();
    for (i, function) in code.0.iter().enumerate() {
        Label::new(format!("Function {}", i)).ui(ui);
        ui.indent("Instructions", |ui| {
            draw_function_instructions(ui, &function.0, i, frame_data);
        });
    }
}

fn draw_function_instructions(
    ui: &mut egui::Ui,
    function: &Function,
    func_id: usize,
    frame_data: &mut BetweenFrameData,
) {
    for (i, instruction) in function.code.iter().enumerate() {
        match instruction {
            Ok((op, pos)) => {
                ui.spacing_mut().indent = 200.0;
                let highlight = BytecodeHighlight {
                    position_bytecode: *pos,
                    selected_token_wat: Some(PositionWat {
                        function: func_id,
                        instruction: i,
                    }),
                    position_info: None,
                    highlight_type: HighlightType::Background,
                    highlight_color: Color32::DARK_GREEN,
                };
                draw_highlightable_wat_label(
                    ui,
                    RichText::new(op.to_string()).text_style(TextStyle::Monospace),
                    highlight,
                    frame_data,
                );
            }
            Err(_) => todo!(),
        }
    }
}

// inspired by: https://github.com/Hirtol/egui_memory_editor
fn draw_bytecode(
    ui: &mut egui::Ui,
    bytecode: &[u8],
    bytecode_option: &BytecodeDisplayOptions,
    frame_data: &mut BetweenFrameData,
) {
    let line_height = ui.text_style_height(&bytecode_option.memory_editor_adress_text_style);
    let max_lines =
        (bytecode.len() + bytecode_option.column_count - 1) / bytecode_option.column_count;

    let mut scroll = ScrollArea::vertical()
        .id_salt(0..0xFFFF)
        .max_height(f32::INFINITY)
        .auto_shrink([false, true]);

    scroll.show_rows(ui, line_height, max_lines, |ui, line_range| {
        egui::Grid::new("bytecode_grid")
            .striped(true)
            .spacing(Vec2::new(15.0, ui.style().spacing.item_spacing.y))
            .show(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                ui.style_mut().spacing.item_spacing.x = 3.0;

                for start_row in line_range.clone() {
                    let start_adress = 0 + (start_row * bytecode_option.column_count);
                    let line_range = start_adress..start_adress + bytecode_option.column_count;
                    draw_bytecode_values(ui, bytecode, start_adress, frame_data);
                    ui.end_row();
                }
            });
    });
}
fn draw_bytecode_values(
    ui: &mut egui::Ui,
    bytecode: &[u8],
    start_adress: usize,
    frame_data: &mut BetweenFrameData,
) {
    let column_count = 16;
    for grid_column in 0..(column_count + 7) / 8 {
        let start_adress = start_adress + 8 * grid_column;

        ui.horizontal(|ui| {
            let column_count = (column_count - 8 * grid_column).min(8);

            for column_index in 0..column_count {
                let memory_adress = start_adress + column_index;

                if memory_adress >= bytecode.len() {
                    break;
                }
                let mem_val = bytecode[memory_adress];

                let label_text = format!("{:02X}", mem_val);

                let mut text = RichText::new(label_text).text_style(TextStyle::Monospace);

                if frame_data.should_highlight(memory_adress) {
                    text = text.background_color(ui.style().visuals.code_bg_color);
                }

                match frame_data.should_highlight_bytecode(memory_adress) {
                    Some((color, hl_type)) => match hl_type {
                        HighlightType::Background => text = text.background_color(color),
                        HighlightType::Border => text = text.color(color).underline(),
                        HighlightType::Bold => text = text.color(color).strong(),
                    },
                    None => (),
                }

                let response = Label::new(text).sense(Sense::click()).ui(ui);

                if response.secondary_clicked() {
                    frame_data.set_highlight_address(memory_adress);
                }

                if response.clicked() {
                    frame_data.set_highlight_address(memory_adress);
                }
            }
        });
    }
}

// TODO: (viv): jedes Highlight eine eigene Farbe (enum mit Farben gebunden am index?)
// Später sollen die Farben über ein Kontextmenü ausgewählt werden können
#[derive(Debug, Clone, PartialEq)]
struct BytecodeHighlight {
    position_bytecode: reader::Position,
    selected_token_wat: Option<PositionWat>,
    position_info: Option<PositionInfo>,
    highlight_type: HighlightType,
    highlight_color: Color32,
}

// TODO: (viv): Highlight auf text anwenden, je nach Typ muss auch die Textfarbe zusätzlich angepasst
// werden, damit der Text lesbar bleibt
impl BytecodeHighlight {
    pub fn apply_highlight(&self, text: RichText) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum HighlightType {
    Background,
    Border,
    Bold,
}

#[derive(Debug, Clone, PartialEq)]
// TODO: (viv): das reicht nicht aus, hier müssen wir uns noch etwas anders überlegen:
// eigener Positionstyp pro SectionType? vermutlich ja...
struct PositionInfo {
    section: SectionType,
    idx: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct PositionWat {
    function: usize,
    instruction: usize,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct BetweenFrameData {
    pub highlight_bytecode: Vec<BytecodeHighlight>,
    pub selected_highlight_address: Option<usize>,
    pub goto_address_string: String,
}

impl BetweenFrameData {
    pub fn set_highlight_address(&mut self, address: usize) {
        self.selected_highlight_address = if matches!(self.selected_highlight_address, Some(current) if current == address)
        {
            self.goto_address_string.clear();
            None
        } else {
            self.goto_address_string = format!("{:X}", address);
            Some(address)
        };
    }

    pub fn toggle_bytecode_highlight(&mut self, highlight: BytecodeHighlight) {
        let index = self.highlight_bytecode.iter().position(|r| *r == highlight);
        match index {
            Some(index) => {
                self.highlight_bytecode.swap_remove(index);
            }
            None => {
                self.highlight_bytecode.push(highlight);
            }
        }
    }

    // TODO: (viv): Farbe des Highlight als Returnwert?
    pub fn should_highlight_info(
        &mut self,
        position_info: &PositionInfo,
    ) -> Option<(Color32, HighlightType)> {
        let mut res = None;
        self.highlight_bytecode
            .iter()
            .for_each(|highlight| match &highlight.position_info {
                Some(info) => {
                    if info.section == position_info.section && info.idx == position_info.idx {
                        res = Some((
                            highlight.highlight_color.clone(),
                            highlight.highlight_type.clone(),
                        ));
                    }
                }
                None => (),
            });
        return res;
    }

    pub fn should_highlight_wat(
        &self,
        position_wat: &PositionWat,
    ) -> Option<(Color32, HighlightType)> {
        let mut should_highlight = None;
        self.highlight_bytecode
            .iter()
            .for_each(|highlight| match &highlight.selected_token_wat {
                Some(pos_wat) => {
                    if pos_wat.function == position_wat.function
                        && pos_wat.instruction == position_wat.instruction
                    {
                        should_highlight = Some((
                            highlight.highlight_color.clone(),
                            highlight.highlight_type.clone(),
                        ));
                    }
                }
                None => (),
            });
        should_highlight
    }

    pub fn should_highlight_bytecode(&self, address: usize) -> Option<(Color32, HighlightType)> {
        let mut res = None;
        for highlight in &self.highlight_bytecode {
            let offset = highlight.position_bytecode.offset;
            let len = highlight.position_bytecode.len;
            if address >= offset && address <= (offset + len - 1) {
                res = Some((
                    highlight.highlight_color.clone(),
                    highlight.highlight_type.clone(),
                ));
            }
        }
        res
    }

    #[inline]
    pub fn should_highlight(&self, address: usize) -> bool {
        self.selected_highlight_address
            .map_or(false, |addr| addr == address)
    }
}
pub struct BytecodeDisplayOptions {
    pub column_count: usize,
    pub zero_color: Color32,
    pub adress_text_color: Color32,
    pub highlight_text_color: Color32,
    pub memory_editor_text_style: TextStyle,
    pub memory_editor_adress_text_style: TextStyle,
    pub(crate) selected_adress_range: String,
}

impl Default for BytecodeDisplayOptions {
    fn default() -> Self {
        BytecodeDisplayOptions {
            column_count: 16,
            zero_color: Color32::from_gray(80),
            adress_text_color: Color32::from_rgb(125, 0, 125),
            highlight_text_color: Color32::from_rgb(0, 140, 140),
            memory_editor_text_style: TextStyle::Monospace,
            memory_editor_adress_text_style: TextStyle::Monospace,
            selected_adress_range: "".to_string(),
        }
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
