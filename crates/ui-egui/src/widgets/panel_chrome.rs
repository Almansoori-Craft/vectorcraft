use super::{Color32, CornerRadius, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Tokens, Ui, Vec2, icons, pos2, theme, vec2};

/// A bordered list box (Swatches tiles, Appearance rows, Artboards list).
pub fn list_box<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE.stroke(Stroke::new(1.0, t.input_border)).inner_margin(egui::Margin::same(1)).show(ui, add).inner
}

/// A bottom button bar separated from the content by a divider (panel footers).
pub fn bottom_bar(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(4.0);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, t.divider);
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        add(ui)
    });
}

/// A row of text tabs, the current one underlined in the accent colour (New Document's
/// categories, Document Setup's sections); in the Al Majlis look, pill tabs in a rounded track
/// ([`pill_tabs`]). Returns the clicked tab's index.
pub fn tab_bar(ui: &mut Ui, tabs: &[&str], current: usize) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    if t.majlis {
        return pill_tabs(ui, &t, tabs, current);
    }
    let mut clicked = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 18.0;
        for (i, tab) in tabs.iter().enumerate() {
            let sel = i == current;
            let font = if sel { theme::semibold(13.0) } else { egui::FontId::proportional(13.0) };
            let galley = ui.painter().layout_no_wrap(tl!(tab).to_string(), font, t.text);
            let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x, 30.0), Sense::click());
            let color = if sel || resp.hovered() { t.text_strong } else { t.text_dim };
            ui.painter().galley(pos2(rect.left(), rect.center().y - galley.size().y / 2.0 - 2.0), galley, color);
            if sel {
                ui.painter().rect_filled(
                    Rect::from_min_max(pos2(rect.left(), rect.bottom() - 2.0), rect.right_bottom()),
                    CornerRadius::same(1),
                    t.accent,
                );
            }
            resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, ui.is_enabled(), sel, tl!(tab)));
            if resp.clicked() {
                clicked = Some(i);
            }
        }
    });
    clicked
}

/// Al Majlis tab strip: a track (raised, 1 pt hairline, radius half its height) holding pill tabs,
/// the current one filled with the accent and white text, the others muted text that turns to the
/// text colour under the pointer. As tall as the underlined row (30 pt).
fn pill_tabs(ui: &mut Ui, t: &Tokens, tabs: &[&str], current: usize) -> Option<usize> {
    let font = egui::FontId::proportional(13.0);
    let galleys: Vec<_> = tabs.iter().map(|tab| ui.painter().layout_no_wrap(tl!(tab).to_string(), font.clone(), t.text)).collect();
    let pad = 12.0;
    let inset = 3.0;
    let width: f32 = galleys.iter().map(|g| g.size().x + 2.0 * pad).sum::<f32>() + 2.0 * inset;
    let (track, _) = ui.allocate_exact_size(vec2(width, 30.0), Sense::hover());
    let pill = CornerRadius::same(t.pill_radius);
    ui.painter().rect_filled(track, pill, t.raised);
    ui.painter().rect_stroke(track, pill, Stroke::new(1.0, t.hairline), StrokeKind::Inside);
    let mut x = track.left() + inset;
    let mut clicked = None;
    for (i, (tab, galley)) in tabs.iter().zip(galleys).enumerate() {
        let sel = i == current;
        let w = galley.size().x + 2.0 * pad;
        let r = Rect::from_min_size(pos2(x, track.top() + inset), vec2(w, track.height() - 2.0 * inset));
        let resp = ui.interact(r, ui.scope_id().with(("pill-tab", i)), Sense::click());
        if sel {
            ui.painter().rect_filled(r, pill, t.accent);
        }
        let color = if sel {
            t.on_accent
        } else if resp.hovered() {
            t.text
        } else {
            t.text_muted
        };
        ui.painter().galley_with_override_text_color(pos2(r.center().x - galley.size().x / 2.0, r.center().y - galley.size().y / 2.0), galley, color);
        resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, ui.is_enabled(), sel, tl!(tab)));
        if resp.clicked() {
            clicked = Some(i);
        }
        x += w;
    }
    clicked
}

/// A page-orientation toggle drawn in code: a portrait or landscape sheet with a folded corner,
/// accent-filled when `selected`. Returns clicked.
pub fn orientation_button(ui: &mut Ui, landscape: bool, selected: bool, tip: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::click());
    if resp.hovered() && !selected {
        ui.painter().rect_filled(rect, t.corner(t.tool_radius, 3), t.hover);
    }
    let size = if landscape { vec2(16.0, 12.0) } else { vec2(12.0, 16.0) };
    let page = Rect::from_center_size(rect.center(), size);
    let color = if selected { t.accent } else { t.icon };
    let fold = 4.0;
    let outline =
        vec![page.left_top(), page.right_top() - vec2(fold, 0.0), page.right_top() + vec2(0.0, fold), page.right_bottom(), page.left_bottom()];
    let fill = if selected { t.accent } else { Color32::TRANSPARENT };
    ui.painter().add(egui::Shape::convex_polygon(outline.clone(), fill, Stroke::new(1.2, color)));
    let corner = [page.right_top() - vec2(fold, 0.0), page.right_top() + vec2(-fold, fold), page.right_top() + vec2(0.0, fold)];
    ui.painter().add(egui::Shape::line(corner.to_vec(), Stroke::new(1.2, if selected { t.panel } else { color })));
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, ui.is_enabled(), selected, tl!(tip)));
    resp.on_hover_text(tl!(tip)).clicked()
}

/// An artboard-layout toggle drawn in code (Rearrange All Artboards): small artboards in the grid
/// cells `tiles` (column, row), tinted, and over them their order as a line from a dot on the first
/// to an arrowhead on the last; accent-drawn in a pressed well when `selected`, greyed in a disabled
/// `ui`. `tip` is shown as it is (the caller translates it). Returns clicked.
pub fn layout_button(ui: &mut Ui, tiles: &[(u8, u8)], selected: bool, tip: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::click());
    let enabled = ui.is_enabled();
    let bg = if selected {
        t.tool_active
    } else if resp.hovered() && enabled {
        t.hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, t.corner(t.tool_radius, 3), bg);
    let color = if !enabled {
        t.text_disabled
    } else if selected {
        t.accent
    } else {
        t.icon
    };
    // Cells at most 10 points a side in a 20-point square: a row's artboards stand tall, a
    // column's lie flat.
    let (cols, rows) = tiles.iter().fold((1u8, 1u8), |(c, r), &(x, y)| (c.max(x.saturating_add(1)), r.max(y.saturating_add(1))));
    let cell = vec2((20.0 / f32::from(cols)).min(10.0), (20.0 / f32::from(rows)).min(10.0));
    let origin = rect.center() - vec2(f32::from(cols) * cell.x, f32::from(rows) * cell.y) / 2.0;
    let centre = |&(x, y): &(u8, u8)| origin + vec2((f32::from(x) + 0.5) * cell.x, (f32::from(y) + 0.5) * cell.y);
    for tile in tiles {
        ui.painter().rect_filled(Rect::from_center_size(centre(tile), cell - vec2(2.0, 2.0)), 1.0, color.gamma_multiply(0.35));
    }
    let path: Vec<Pos2> = tiles.iter().map(centre).collect();
    if let ([first, ..], [.., from, to]) = (path.as_slice(), path.as_slice()) {
        let (first, from, to) = (*first, *from, *to);
        let dir = (to - from).normalized();
        let side = dir.rot90() * 2.5;
        let head = vec![to + dir * 1.5, to - dir * 2.5 + side, to - dir * 2.5 - side];
        ui.painter().circle_filled(first, 1.6, color);
        ui.painter().add(egui::Shape::line(path, Stroke::new(1.2, color)));
        ui.painter().add(egui::Shape::convex_polygon(head, color, Stroke::NONE));
    }
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, enabled, selected, tip));
    resp.on_hover_text(tip).clicked()
}

/// The chevron cell on the right of a field that opens its preset list (rounded on the right).
pub(super) fn chevron_cell(ui: &mut Ui, h: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (dr, dresp) = ui.allocate_exact_size(vec2(20.0, h), Sense::click());
    let r = t.corner(t.field_radius, 2).ne;
    ui.painter().rect_filled(dr, CornerRadius { nw: 0, sw: 0, ne: r, se: r }, t.input);
    ui.painter().rect_stroke(dr, CornerRadius { nw: 0, sw: 0, ne: r, se: r }, Stroke::new(1.0, t.input_border), StrokeKind::Inside);
    let c = if !ui.is_enabled() {
        t.text_disabled
    } else if dresp.hovered() {
        t.text_strong
    } else {
        t.icon
    };
    icons::paint(ui, "chevron-down", Rect::from_center_size(dr.center(), Vec2::splat(12.0)), c);
    dresp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, ui.is_enabled(), tl!("Presets")));
    dresp
}
