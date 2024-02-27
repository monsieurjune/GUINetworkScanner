import customtkinter as ctk

from sidebar import Sidebar
from navbar import Navbar
from content import Content


class App(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Network Scanner Tool with Rust")
        self.geometry("1024x768")
        self.color = "dark"
        ctk.set_appearance_mode(self.color)

        self.grid_rowconfigure(0, weight=0)
        self.grid_rowconfigure(1, weight=1)
        self.grid_columnconfigure(0, weight=0)
        self.grid_columnconfigure(1, weight=1)

        self.navbar = Navbar(self)
        self.navbar.grid(row=0, column=0, columnspan=2, sticky="nsew")

        self.sidebar = Sidebar(self)
        self.sidebar.grid(row=1, column=0, sticky="nsew")

        self.content = Content(self)


if __name__ == "__main__":
    app = App()
    app.mainloop()
