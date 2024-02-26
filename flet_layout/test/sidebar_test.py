from flet import (
    alignment,
    Checkbox,
    colors,
    Column,
    Container,
    CrossAxisAlignment,
    dropdown,
    DataCell,
    DataColumn,
    DataRow,
    DataTable,
    Divider,
    ElevatedButton,
    page,
    Row,
    Text,
    TextField,
    UserControl,
)


# class Sidebar(UserControl):
#     def __init__(self):
#         super().__init__()

#         self.sidebar_height = 600
#         self.sidebar_width = 350

#         self.ip_address = TextField(
#             label="IP Address...",
#             height=60,
#             width=240,
#             border_radius=10,
#         )
#         self.ip_addr = Text("")

#         self.ip_address_list = DataTable(
#             columns=[
#                 DataColumn(Checkbox(value=False)),
#                 DataColumn(Text("IP Address")),
#                 DataColumn(Text("MAC Address")),
#             ],
#             rows=[],
#         )

#         self.scan_results_list = DataTable(
#             columns=[
#                 DataColumn(Checkbox(value=False)),
#                 DataColumn(Text("IP Address")),
#                 DataColumn(Text("MAC Address")),
#             ],
#             rows=[],
#         )

#         self.interface_list = [
#             dropdown.Option("WLAN"),
#             dropdown.Option("vEthernet"),
#             dropdown.Option("Loopback"),
#             dropdown.Option("Bluetooth"),
#         ]

#         self.probe_button = ElevatedButton(
#             text="Probe",
#             bgcolor="blue",
#             color="white",
#             height=60,
#             width=150,
#         )

#         def edit_table(event, selected):
#             self.ip_address.value = selected
#             self.ip_addr.value = int(event)

#         def add_button(event):
#             self.ip_address_list.rows.append(
#                 DataRow(
#                     cells=[
#                         DataCell(
#                             Checkbox(
#                                 value=False,
#                                 on_change=edit_table,
#                             )
#                         ),
#                         DataCell(Text(self.ip_address.value)),
#                         DataCell(Text("AB:CD:EF:12:34:56")),
#                     ],
#                     on_select_changed=lambda event: edit_table(
#                         event.control.cells[0].content.value,
#                         event.control.cells[1].content.value,
#                         event.control.cells[2].content.value,
#                     ),
#                 )
#             )

#             self.ip_address.value = ""

#             page.update()

#         self.scan_button = ElevatedButton(
#             text="Add IP Address",
#             bgcolor="blue",
#             color="white",
#             height=60,
#             width=150,
#             on_click=add_button,
#         )

#     def build(self):
#         self.view = Container(
#             content=Row(
#                 [
#                     Container(
#                         bgcolor=colors.BLACK26,
#                         border_radius=30,
#                         height=self.sidebar_height,
#                         alignment=alignment.center_right,
#                         width=2,
#                     ),
#                 ],
#                 expand=True,
#                 vertical_alignment=CrossAxisAlignment.START,
#             )
#         )
#         return self.view


class Sidebar(UserControl):
    def __init__(self):
        super().__init__()

        self.sidebar_width = 350
        self.sidebar_height = 600

        self.sidebar_items = [Container(Column([Row([])]))]

    def ip_address_input(self):
        return TextField(
            label="IP Address...",
            height=60,
            width=240,
            border_radius=10,
        )

    def interface_dropdown(self):
        return [
            dropdown.Option("WLAN"),
            dropdown.Option("vEthernet"),
            dropdown.Option("Loopback"),
            dropdown.Option("Bluetooth"),
        ]

    def add_button(self):
        return self.ip_address_input
