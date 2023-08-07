import 'package:flutter/material.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Row(children: <Widget>[
          Container(
            margin: const EdgeInsets.only(right: 5),
            child: const Icon(Icons.cookie),
          ),
          Text(widget.title)
        ]),
      ),
      body: Center(
        child: Column(
          children: <Widget>[
            Container(
                height: 50,
                width: 210,
                padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
                margin: const EdgeInsets.only(top: 100.0),
                child: ElevatedButton(
                    child: Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: const <Widget>[
                          Icon(Icons.person),
                          Text("Create Account"),
                        ]),
                    onPressed: () async {
                      Navigator.pushNamed(context, '/create-account');
                    })),
            Container(
                height: 50,
                width: 210,
                padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
                margin: const EdgeInsets.only(top: 10.0),
                child: ElevatedButton(
                    child: Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: const <Widget>[
                          Icon(Icons.lock_open),
                          Text("Account Login"),
                        ]),
                    onPressed: () async {
                      Navigator.pushNamed(context, '/login');
                    }))
          ],
        ),
      ),
    );
  }
}
