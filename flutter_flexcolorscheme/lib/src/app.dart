import 'package:flutter/material.dart';
import 'package:flex_color_scheme/flex_color_scheme.dart';
import "./statefulbutton.dart";

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: FlexThemeData.light(scheme: FlexScheme.material),
      // The Mandy red, dark theme.
      darkTheme: FlexThemeData.dark(scheme: FlexScheme.material),
      routes: {
        ExtractArgumentsScreen.routeName: ExtractArgumentsScreen.routeBuilder,
      },
      title: 'Navigation with Arguments',
      home: const HomeScreen(),
    );
  }
}

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Home Screen'),
        actions: <Widget>[
          IconButton(
            icon: const Icon(Icons.menu),
            tooltip: 'Open shopping cart',
            onPressed: () {
              print("Clicked");
              // handle the press
            },
          ),
        ],
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            // A button that navigates to a named route.
            // The named route extracts the arguments
            // by itself.
            ElevatedButton(
              onPressed: () {
                // When the user taps the button,
                // navigate to a named route and
                // provide the arguments as an optional
                // parameter.
                Navigator.pushNamed(
                  context,
                  ExtractArgumentsScreen.routeName,
                  arguments: ScreenArguments(
                    'Extract Arguments Screen',
                    'This message is extracted in the build method.',
                  ),
                );
              },
              child: const Text('Extract 1 argument'),
            ),
          ],
        ),
      ),
    );
  }
}

// A Widget that extracts the necessary arguments from
// the ModalRoute.
class ExtractArgumentsScreen extends StatelessWidget {
  static WidgetBuilder routeBuilder =
      (context) => const ExtractArgumentsScreen();

  const ExtractArgumentsScreen({super.key});

  static const routeName = '/extractArguments';

  @override
  Widget build(BuildContext context) {
    // Extract the arguments from the current ModalRoute
    // settings and cast them as ScreenArguments.
    final args = ModalRoute.of(context)!.settings.arguments as ScreenArguments;

    return Scaffold(
      appBar: AppBar(
        title: Text(args.title),
      ),
      body: Center(
        child: Flex(
          direction: Axis.vertical,
          children: <Widget>[
            Text(
              args.title,
              style: const TextStyle(fontSize: 48),
            ),
            Text(
              args.message,
              style: const TextStyle(fontSize: 28),
            ),
            StatefulButton(
              key: key,
            )
          ],
        ),
      ),
    );
  }
}

class ScreenArguments {
  final String title;
  final String message;

  ScreenArguments(this.title, this.message);
}
