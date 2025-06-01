use eframe::egui;
use egui::{Painter, Ui, Pos2, Stroke, Response, Context, epaint::PathShape}; // Removed unused Color32, Vec2
use std::f32::consts::PI;
use crate::models::ExpenseCategory;

pub fn draw_pie_chart(
    _ui: &mut Ui, // Ui might be needed for adding other elements around/on the chart, or for Id generation
    painter: Painter, // Changed from &mut Painter to Painter as painter_at returns Painter
    center: Pos2,
    radius: f32,
    response: &Response, // For hover detection
    expenses: &[ExpenseCategory],
    ctx: &Context, // For tooltips
    chart_id_source: &str, // Added for unique ID generation
) {
    let total_amount: f32 = expenses.iter().map(|e| e.amount).sum();
    // Assuming total_amount > 0 because this function is called only if it is.

    let mut start_angle = -PI / 2.0; // Start from the top
    let mut hover_texts = Vec::new();

    for expense in expenses.iter() { // Removed 'index' as it was unused
        let proportion = expense.amount / total_amount;
        let angle_delta = proportion * 2.0 * PI;
        let end_angle = start_angle + angle_delta;

        let mut points = vec![center];
        let num_segments = (angle_delta * radius / 5.0).ceil().max(10.0) as usize;
        for i in 0..=num_segments {
            let angle = start_angle + (angle_delta * i as f32 / num_segments as f32);
            points.push(Pos2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin()));
        }
        points.push(center);

        let shape = PathShape::convex_polygon(points.clone(), expense.color, Stroke::NONE);
        painter.add(shape);

        // Prepare text for legend/hover
        // Note: hover_pos was unused, direct segment hover detection is more complex.
        // Tooltip is shown based on overall chart rect hover.
        hover_texts.push(format!("{}: {:.2} ({:.1}%)", expense.name, expense.amount, proportion * 100.0));
        
        start_angle = end_angle;
    }

    // Display hover information if mouse is over the chart
    if response.hovered() {
        let tooltip_id = egui::Id::new(format!("{}_tooltip", chart_id_source));
        egui::show_tooltip_at_pointer(ctx, tooltip_id, |ui_tooltip| {
            for (i, text) in hover_texts.iter().enumerate() {
                 ui_tooltip.colored_label(expenses[i].color, text);
            }
        });
    }
}
