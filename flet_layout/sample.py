import flet
from flet import (
    Text,
    ElevatedButton,
    Column,
    Row,
    Page,
)


def main(page: Page):
    hist = []

    def add_element(e):
        value = e.control.data["key"]
        hist.append(Text(value=f"ELEMENNT ADDED {value}"))
        add_elements.controls.clear()
        add_elements.controls.append(hist[-1])
        page.update()

    def del_element(e):
        if hist:
            hist.pop()
            add_elements.controls.clear()
            page.update()
        if hist:
            add_elements.controls.append(hist[-1])
            page.update()

    btn_1 = ElevatedButton(text="Button1", on_click=add_element, data={"key": "value1"})
    btn_2 = ElevatedButton(text="Button2", on_click=add_element, data={"key": "value2"})
    btn_3 = ElevatedButton(text="Button3", on_click=add_element, data={"key": "value3"})
    btn_del = ElevatedButton(text="Delete", on_click=del_element)

    add_elements = Column()

    layout = Column(
        controls=[
            Row(
                controls=[btn_1, btn_2, btn_3],
                alignment="center",
            ),
            btn_del,
            add_elements,
        ]
    )
    page.add(layout)
    page.update()


flet.app(target=main)
