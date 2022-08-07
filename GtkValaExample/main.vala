int main (string[] argv) {
    // Create a new application
    var app = new Gtk.Application ("com.example.GtkApplication", GLib.ApplicationFlags.FLAGS_NONE);

    app.activate.connect (() => {
        // Create a new window
        var window = new Gtk.ApplicationWindow (app);
        window.set_default_size (300, 100);

        // Create a new button
        var button = new Gtk.Button.with_label ("Click me!");

        // When the button is clicked, close the window
        int count = 0;
        button.clicked.connect (() => {
            count++;
            button.label = "You Clicked " + count.to_string () + " Times";
        });

        window.set_child (button);
        window.present ();
    });

    return app.run (argv);
}
