#[test]
fn bevy_issue_9530() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy . new_leaf_with_measure (taffy :: style :: Style { align_items : Some (taffy :: style :: AlignItems :: Center) , align_content : Some (taffy :: style :: AlignContent :: Center) , justify_content : Some (taffy :: style :: JustifyContent :: Center) , flex_grow : 1f32 , margin : taffy :: geometry :: Rect { left : taffy :: style :: LengthPercentageAuto :: Length (20f32) , right : taffy :: style :: LengthPercentageAuto :: Length (20f32) , top : taffy :: style :: LengthPercentageAuto :: Length (20f32) , bottom : taffy :: style :: LengthPercentageAuto :: Length (20f32) , } , .. Default :: default () } , taffy :: tree :: MeasureFunc :: Raw (| known_dimensions , available_space | { const TEXT : & str = "HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH" ; crate :: measure_standard_text (known_dimensions , available_space , TEXT , crate :: WritingMode :: Horizontal , None) }) ,) . unwrap () ;
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(20f32),
                    right: taffy::style::LengthPercentageAuto::Length(20f32),
                    top: taffy::style::LengthPercentageAuto::Length(20f32),
                    bottom: taffy::style::LengthPercentageAuto::Length(20f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(20f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(20f32),
                    bottom: taffy::style::LengthPercentage::Length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(300f32),
                    height: taffy::style::Dimension::Length(300f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Auto,
                    right: taffy::style::LengthPercentageAuto::Auto,
                    top: taffy::style::LengthPercentageAuto::Auto,
                    bottom: taffy::style::LengthPercentageAuto::Auto,
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node, 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node0, 300f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node0, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node1, 300f32, size.width);
    assert_eq!(size.height, 420f32, "height of node {:?}. Expected {}. Actual {}", node1, 420f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 260f32, "width of node {:?}. Expected {}. Actual {}", node10, 260f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node10, 50f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node10, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node10, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11).unwrap();
    assert_eq!(size.width, 220f32, "width of node {:?}. Expected {}. Actual {}", node11, 220f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node11, 240f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node11, 40f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node11, 90f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node12).unwrap();
    assert_eq!(size.width, 260f32, "width of node {:?}. Expected {}. Actual {}", node12, 260f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node12, 50f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node12, 20f32, location.x);
    assert_eq!(location.y, 350f32, "y of node {:?}. Expected {}. Actual {}", node12, 350f32, location.y);
}
