use eframe::egui;
use egui::{Painter, Ui, Pos2, Stroke, Response, Context, epaint::PathShape}; // Removed unused Color32, Vec2
use std::f32::consts::PI;
use crate::models::ExpenseCategory;

pub fn draw_pie_chart(
    _ui: &mut Ui, // Ui might be needed for adding other elements around/on the chart, or for Id generation
    painter: Painter, // Changed from &mut Painter to Painter as painter_at returns Painter
    center: Pos2,
    radius: f32,
    response: &Response, // For overall chart area hover detection
    expenses: &[ExpenseCategory],
    ctx: &Context, // For tooltips
    chart_id_source: &str,
    current_hover_idx: Option<usize>, // Which segment is currently considered hovered by the app
) -> Option<usize> { // Returns the index of the segment hovered in this frame, if any
    let total_amount: f32 = expenses.iter().map(|e| e.amount).sum();
    if total_amount == 0.0 { return None; }

    let mut start_angle = -PI / 2.0;
    let mut newly_hovered_idx: Option<usize> = None;
    let hover_effect_radius_increase: f32 = radius * 0.05; // 5% increase for hovered segment

    let mouse_pos = response.hover_pos(); // Absolute mouse position

    for (idx, expense) in expenses.iter().enumerate() {
        let proportion = expense.amount / total_amount;
        let angle_delta = proportion * 2.0 * PI;
        let end_angle = start_angle + angle_delta;
        let mid_angle = start_angle + angle_delta / 2.0;

        let mut current_radius = radius;
        if current_hover_idx == Some(idx) {
            current_radius += hover_effect_radius_increase;
        }

        // Create path for the segment
        let mut points = vec![center];
        let num_segments = (angle_delta * current_radius / 4.0).ceil().max(10.0) as usize; // More segments for larger radius
        for i in 0..=num_segments {
            let angle = start_angle + (angle_delta * i as f32 / num_segments as f32);
            points.push(Pos2::new(center.x + current_radius * angle.cos(), center.y + current_radius * angle.sin()));
        }
        points.push(center);
        
        let segment_shape = PathShape::convex_polygon(points.clone(), expense.color, Stroke::NONE);
        painter.add(segment_shape);

        // Precise hover detection for this segment
        if let Some(abs_mouse_pos) = mouse_pos {
            if response.rect.contains(abs_mouse_pos) { // Mouse is within the chart's bounding box
                let mouse_relative_to_center = abs_mouse_pos - center;
                let dist_sq = mouse_relative_to_center.length_sq();

                if dist_sq <= radius.powi(2) { // Check if within the original radius for interaction
                    let mut mouse_angle = mouse_relative_to_center.y.atan2(mouse_relative_to_center.x);
                    // Normalize angles to be consistently positive or within a clear range if needed,
                    // start_angle and end_angle are typically in [-PI, PI] or [0, 2PI]
                    // atan2 returns in [-PI, PI]. Ensure start_angle/end_angle are compatible.
                    // A common way to handle angle ranges:
                    // (Ensure all angles are in [0, 2PI) or [-PI, PI) consistently)
                    // For simplicity, assuming angles are fine for now. More robust check needed for edge cases.
                    
                    // Normalize mouse_angle to be in the same range as start_angle and end_angle
                    // (e.g. if start_angle can go beyond PI or below -PI due to accumulation)
                    // This basic check works if angle_delta is not > PI and angles are within a 2PI range.
                    let mut normalized_start = start_angle;
                    let mut normalized_end = end_angle;
                    let mut normalized_mouse_angle = mouse_angle;

                    // Example normalization to [0, 2PI) - can be complex with wrapping
                    // For now, a simpler check assuming angles are somewhat well-behaved:
                    // This check is basic and might fail near the 0/2PI transition if not handled carefully.
                    // A robust way involves checking if (mouse_angle - start_angle) mod 2PI < (end_angle - start_angle) mod 2PI
                    
                    // A simpler geometric test: is the point in the angular sector?
                    // Check if the mouse angle is between start and end angles.
                    // This needs to handle wrapping around the 2PI boundary.
                    // A common technique is to check cross products or compare normalized angles.
                    // For now, let's assume a direct comparison works for most cases if angles are ordered.
                    // This is a known tricky part of pie chart interaction.

                    // Let's use a simpler check: if the mouse is inside the polygon shape.
                    // However, egui doesn't directly provide point-in-polygon for PathShape for interaction.
                    // We'll rely on the angle check, but it needs to be robust.
                    // A simple way: if start_angle < end_angle (normal case)
                    //   is mouse_angle >= start_angle && mouse_angle < end_angle
                    // if start_angle > end_angle (wrapped around 2PI)
                    //   is mouse_angle >= start_angle || mouse_angle < end_angle
                    // This still needs careful normalization of all angles to a consistent [0, 2PI) range.

                    // For now, we'll set newly_hovered_idx if mouse is in the bounding box of the chart
                    // and then the tooltip will show all. A more precise hit-test is an enhancement.
                    // Let's try a basic angle check (may not be perfect):
                    // Normalize all angles to [0, 2*PI)
                    fn normalize_angle(angle: f32) -> f32 {
                        let mut norm = angle % (2.0 * PI);
                        if norm < 0.0 { norm += 2.0 * PI; }
                        norm
                    }
                    let sa_norm = normalize_angle(start_angle);
                    let ea_norm = normalize_angle(end_angle);
                    let ma_norm = normalize_angle(mouse_angle);

                    if sa_norm <= ea_norm { // Normal case
                        if ma_norm >= sa_norm && ma_norm < ea_norm {
                            newly_hovered_idx = Some(idx);
                        }
                    } else { // Wrapped around 0/2PI
                        if ma_norm >= sa_norm || ma_norm < ea_norm {
                            newly_hovered_idx = Some(idx);
                        }
                    }
                }
            }
        }
        start_angle = end_angle;
    }

    // Display tooltip for the specifically hovered segment
    if let Some(hover_idx) = current_hover_idx { // Use the state passed from app
        if hover_idx < expenses.len() { // Check bounds
            let expense = &expenses[hover_idx];
            let proportion = expense.amount / total_amount;
            let tooltip_text = format!("{}: {:.2} ({:.1}%)", expense.name, expense.amount, proportion * 100.0);
            let tooltip_id = egui::Id::new(format!("{}_segment_tooltip_{}", chart_id_source, hover_idx));
            egui::show_tooltip_at_pointer(ctx, tooltip_id, |ui_tooltip| {
                ui_tooltip.colored_label(expense.color, tooltip_text);
            });
        }
    }
    
    newly_hovered_idx // Return the segment index hovered in this frame
}
