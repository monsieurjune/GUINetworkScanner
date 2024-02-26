import flet
from flet import (
    DataColumn,
    DataTable,
    Page,
    Row,
    Text,
)

from header_test import Header
from sidebar_test import Sidebar


def main(page: Page):
    page.title = "Network Scanner Tool with Rust"
    page.padding = 15

    Header(page)
    sidebar = Sidebar()

    scan_results = DataTable(
        columns=[
            DataColumn(Text("#")),
            DataColumn(Text("Protocol")),
            DataColumn(Text("Port")),
            DataColumn(Text("Description")),
            DataColumn(Text("Status")),
        ],
        rows=[],
    )

    layout = Row(
        controls=[sidebar, scan_results],
        tight=True,
        expand=True,
        vertical_alignment="start",
    )

    page.add(layout)
    page.update()


if __name__ == "__main__":
    flet.app(target=main)
