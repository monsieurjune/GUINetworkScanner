import customtkinter as ctk

from navbar import Navbar
from sidebar import Sidebar
from content import Content


class App(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Network Scanner Tool with Rust")
        self.geometry("1024x768")
        self.color = "dark"

        self.ip_addresses = []
        self.interfaces = []
        self.scan_result = []

        self.grid_rowconfigure(0, weight=0)
        self.grid_rowconfigure(1, weight=1)
        self.grid_columnconfigure(0, weight=0)
        self.grid_columnconfigure(1, weight=1)

        Navbar(self)
        Sidebar(self)
        Content(self)


if __name__ == "__main__":
    app = App()
    app.mainloop()
