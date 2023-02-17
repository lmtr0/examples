import "package:flutter/material.dart";

class StatefulButton extends StatefulWidget {
  const StatefulButton({super.key});

  @override
  State<StatefulWidget> createState() => _StatefulButton();
}

class _StatefulButton extends State<StatefulButton> {
  int count = 0;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: () {
        setState(() {
          count++;
        });
      },
      child: Text("Click me $count times!"),
    );
  }
}
