import customtkinter as ctk

from __sidebar import Sidebar
from __header import Header

from __content import Content


class App(ctk.CTk):
    def __init__(self) -> None:
        super().__init__()

        self.title(string="Network Scanner Tool with Rust")
        self.geometry(geometry_string="1280x720")
        self.resizable(width=False, height=False)
        self.color = "dark"
        ctk.set_appearance_mode(mode_string=self.color)

        self.grid_rowconfigure(index=0, weight=0)
        self.grid_rowconfigure(index=1, weight=1)
        self.grid_columnconfigure(index=0, weight=0)
        self.grid_columnconfigure(index=1, weight=1)

        self.navbar = Header(master=self)
        self.navbar.grid(row=0, column=0, columnspan=2, sticky="nsew")

        self.sidebar = Sidebar(master=self)
        self.sidebar.grid(row=1, column=0, sticky="nsew")

        self.content = Content(master=None)


if __name__ == "__main__":
    app = App()
    app.mainloop()
