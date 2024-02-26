import flet
from flet import *


def main(page: Page):
    page.scroll = "always"
    page.theme_mode = ThemeMode.LIGHT

    ip_address = TextField(
        label="IP Address...",
        height=60,
        width=240,
        border_radius=10,
    )
    ip_addr = Text("")

    ip_address_lists = DataTable(
        columns=[
            DataColumn(Checkbox(value=False)),
            DataColumn(Text("IP Address")),
            DataColumn(Text("MAC Address")),
        ],
        rows=[],
    )

    scan_results_lists = DataTable(
        columns=[
            DataColumn(Checkbox(value=False)),
            DataColumn(Text("IP Address")),
            DataColumn(Text("MAC Address")),
        ],
        rows=[],
    )

    interface_info = [
        dropdown.Option("eth0"),
        dropdown.Option("lo"),
        dropdown.Option("blu"),
    ]

    interface_list = Dropdown(
        options=interface_info,
        height=60,
        width=240,
        border_radius=10,
    )

    def edit_table(event, selected):
        ip_address.value = selected
        ip_addr.value = int(event)

        add_button.visible = False

        page.update()

    def add_button(event):
        ip_address_lists.rows.append(
            DataRow(
                cells=[
                    DataCell(Checkbox(value=False, on_change=edit_table)),
                    DataCell(Text(ip_address.value)),
                    DataCell(Text("B6-F7-AE-9E-98-4B")),
                ],
                on_select_changed=lambda event: edit_table(
                    event.control.cells[0].content.value,
                    event.control.cells[1].content.value,
                    event.control.cells[2].content.value,
                ),
            )
        )

        ip_address.value = ""
        page.update()

    scan_button = ElevatedButton(
        "Add IP Address",
        bgcolor="blue",
        color="white",
        height=55,
        width=150,
        on_click=add_button,
    )

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

    def scan_result_button(event):
        scan_results_lists.rows.append(
            DataRow(
                cells=[
                    DataCell(Checkbox(value=False, on_change=edit_table)),
                    DataCell(Text(ip_address.value)),
                    DataCell(Text("B6-F7-AE-9E-98-4B")),
                ],
                on_select_changed=lambda event: edit_table(
                    event.control.cells[0].content.value,
                    event.control.cells[1].content.value,
                    event.control.cells[2].content.value,
                ),
            )
        )

        ip_address.value = ""
        page.update()

    def interface_probe(): ...

    probe_button = ElevatedButton(
        "Probe",
        bgcolor="green",
        color="white",
        height=55,
        width=150,
    )

    page.add(
        Row(
            [
                Column(
                    [
                        Text(
                            "Network Scanner Tool with Rust",
                            size=30,
                            weight="bold",
                        ),
                        Row(
                            [
                                ip_address,
                                scan_button,
                            ],
                            alignment="start",
                        ),
                        Row(
                            [
                                interface_list,
                                probe_button,
                            ],
                            alignment="start",
                        ),
                        Divider(height=7, thickness=5),
                        ip_address_lists,
                    ]
                ),
                Divider(),
                Column(
                    [
                        Text("Scan Results", size=18, weight="bold"),
                        scan_results,
                    ],
                ),
            ],
        ),
        Row(
            [
                ElevatedButton(
                    "Settings",
                    bgcolor="gray",
                    color="white",
                    height=50,
                    width=100,
                ),
            ],
        ),
    )


flet.app(target=main)
