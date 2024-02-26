from flet import (
    AppBar,
    ButtonStyle,
    colors,
    Container,
    ElevatedButton,
    IconButton,
    margin,
    Page,
    Row,
    RoundedRectangleBorder,
    Text,
    TextThemeStyle,
    VerticalDivider,
    UserControl,
)


class Header(UserControl):
    def __init__(self, page: Page):
        super().__init__()

        self.page = page
        self.page.theme_mode = "light"
        self.toggle_dark_light_icon = IconButton(
            icon="dark_mode",
            width=69,
            height=69,
            selected_icon="light_mode",
            tooltip="switch theme mode",
            on_click=self.toggle_icon,
        )
        self.home_page_button = ElevatedButton(
            text="Home Page",
            bgcolor=colors.ORANGE,
            color=colors.WHITE,
            height=69,
            style=ButtonStyle(
                shape=RoundedRectangleBorder(radius=10),
            ),
            on_click=lambda _: self.page.go("/"),
        )
        self.history_logs_button = ElevatedButton(
            text="History Logs",
            bgcolor=colors.GREEN,
            color=colors.WHITE,
            height=69,
            style=ButtonStyle(
                shape=RoundedRectangleBorder(radius=10),
            ),
            on_click=lambda _: self.page.go("/history"),
        )

        self.page.appbar = AppBar(
            leading_width=100,
            title=Text(
                value="Network Scanner Tool with Rust",
                size=46,
                text_align="left",
                theme_style=TextThemeStyle.TITLE_LARGE,
            ),
            center_title=False,
            toolbar_height=125,
            bgcolor=colors.SURFACE_VARIANT,
            actions=[
                Container(
                    content=Row(
                        [
                            Row(
                                [
                                    self.home_page_button,
                                    self.history_logs_button,
                                ],
                                spacing=25,
                            ),
                            VerticalDivider(width=50),
                            self.toggle_dark_light_icon,
                        ],
                        alignment="spaceBetween",
                    ),
                    margin=margin.only(left=50, right=25),
                )
            ],
        )

    def build(self):
        return self.page.appbar

    def toggle_icon(self, event):
        self.page.theme_mode = "light" if self.page.theme_mode == "dark" else "dark"
        self.page.update()
        self.toggle_dark_light_icon.selected = not self.toggle_dark_light_icon.selected
        self.page.update()

    def go_history(self, event):
        self.page.route = "/history"
        self.page.update()
        print("Navigating to Page: History")
