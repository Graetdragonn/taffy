#[test]
fn grid_absolute_row_start() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            grid_row: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Line(1i16),
                end: taffy::style::GridPlacement::Auto,
            },
            position: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(4f32),
                right: taffy::style::LengthPercentageAuto::Points(3f32),
                top: taffy::style::LengthPercentageAuto::Points(1f32),
                bottom: taffy::style::LengthPercentageAuto::Points(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
                grid_template_columns: vec![points(40f32), points(40f32), points(40f32)],
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(40f32),
                    right: taffy::style::LengthPercentage::Points(20f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(30f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 180f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 173f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 173f32, size.width);
    assert_eq!(size.height, 147f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 147f32, size.height);
    assert_eq!(location.x, 4f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 4f32, location.x);
    assert_eq!(location.y, 11f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 11f32, location.y);
}