from flet import (
    alignment,
    border_radius,
    colors,
    Container,
    CrossAxisAlignment,
    FloatingActionButton,
    Icon,
    IconButton,
    icons,
    NavigationRail,
    NavigationRailDestination,
    NavigationRailLabelType,
    Row,
    Text,
    UserControl,
)


class Sidebar(UserControl):
    def __init__(self):
        super().__init__()

        self.nav_rail_visible = True
        self.nav_rail_items = [
            NavigationRailDestination(
                icon_content=Icon(icons.BOOKMARK_BORDER),
                selected_icon_content=Icon(icons.BOOKMARK),
                label="Second",
            ),
            NavigationRailDestination(
                icon=icons.FAVORITE_BORDER, selected_icon=icons.FAVORITE, label="First"
            ),
            NavigationRailDestination(
                icon=icons.SETTINGS_OUTLINED,
                selected_icon_content=Icon(icons.SETTINGS),
                label_content=Text("Settings"),
            ),
        ]
        self.nav_rail = NavigationRail(
            height=300,
            selected_index=None,
            label_type=NavigationRailLabelType.ALL,
            min_width=100,
            min_extended_width=400,
            leading=FloatingActionButton(icon=icons.CREATE, text="ADD"),
            group_alignment=-0.9,
            destinations=self.nav_rail_items,
            on_change=lambda e: print(
                "Selected destination: ", e.control.selected_index
            ),
        )
        self.toggle_nav_rail_button = IconButton(
            icon=icons.ARROW_CIRCLE_LEFT,
            icon_color=colors.BLUE_GREY_400,
            selected=False,
            selected_icon=icons.ARROW_CIRCLE_RIGHT,
            on_click=self.toggle_nav_rail,
            tooltip="Collapse Nav Bar",
        )

    def build(self):
        self.view = Container(
            content=Row(
                [
                    self.nav_rail,
                    Container(  # vertical divider
                        bgcolor=colors.BLACK26,
                        border_radius=border_radius.all(30),
                        height=220,
                        alignment=alignment.center_right,
                        width=2,
                    ),
                    self.toggle_nav_rail_button,
                ],
                expand=True,
                vertical_alignment=CrossAxisAlignment.START,
                visible=self.nav_rail_visible,
            )
        )
        return self.view

    def toggle_nav_rail(self, e):
        self.nav_rail.visible = not self.nav_rail.visible
        self.toggle_nav_rail_button.selected = not self.toggle_nav_rail_button.selected
        self.toggle_nav_rail_button.tooltip = (
            "Open Side Bar"
            if self.toggle_nav_rail_button.selected
            else "Collapse Side Bar"
        )
        self.view.update()
        self.page.update()
