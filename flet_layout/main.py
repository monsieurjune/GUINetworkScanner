import flet
from flet import (
    Page,
    Row,
    Text,
)

from header import AppHeader
from sidebar import Sidebar


def main(page: Page):
    page.title = "Network Scanner Tool with Rust"
    page.padding = 10
    my_text = Text("Scan Results")

    AppHeader(page)
    sidebar = Sidebar()
    layout = Row(
        controls=[sidebar, my_text],
        tight=True,
        expand=True,
        vertical_alignment="start",
    )

    page.add(layout)
    page.update()


if __name__ == "__main__":
    flet.app(target=main)
