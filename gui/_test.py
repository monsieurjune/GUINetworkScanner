import tkinter as tk
from tkinter import ttk


def handle_button_click():
    # Replace this with the action to perform when the button is clicked
    print("Button clicked!")


def handle_combobox_selection(event):
    # Get the selected value from the combobox
    selected_value = combobox.get()
    print(f"You selected: {selected_value}")


# Create the main window
window = tk.Tk()
window.title("Modern Python GUI")

# Left side widgets
entry_label = tk.Label(window, text="Enter text:")
entry_label.grid(row=0, column=0, sticky="w")  # Sticky for left alignment

entry = tk.Entry(window)
entry.grid(row=0, column=1)

combobox_label = tk.Label(window, text="Select an option:")
combobox_label.grid(row=1, column=0, sticky="w")

combobox_values = ["Option 1", "Option 2", "Option 3"]
combobox = ttk.Combobox(window, values=combobox_values)
combobox.current(0)  # Set initial selection
combobox.bind("<<ComboboxSelected>>", handle_combobox_selection)
combobox.grid(row=1, column=1)

left_button = tk.Button(window, text="Left Button", command=handle_button_click)
left_button.grid(row=2, column=0, columnspan=2)  # Span two columns

# Right side widgets (can add as needed, similar structure)
right_label = tk.Label(window, text="Right Side")
right_label.grid(row=0, column=3, sticky="e")  # Sticky for right alignment

# Start the GUI
window.mainloop()
