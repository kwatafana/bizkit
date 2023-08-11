import 'package:flutter/material.dart';
import 'package:kong/kong.dart';
import 'package:omatala/omatala.dart';

// Dashboard Screen
class DashboardScreen extends StatefulWidget {
  const DashboardScreen(
      {Key? key,
      required this.title,
      required this.kong,
      required this.omatala})
      : super(key: key);
  final KongAPI kong;
  final OmatalaAPI omatala;
  final String title;

  @override
  State<DashboardScreen> createState() =>
      _DashboardScreenState(kong: kong, omatala: omatala);
}

class _DashboardScreenState extends State<DashboardScreen> {
  _DashboardScreenState({required this.kong, required this.omatala});
  final KongAPI kong;
  final OmatalaAPI omatala;

  @override
  Widget build(BuildContext context) {
    return DefaultTextStyle(
      style: Theme.of(context).textTheme.displayMedium!,
      textAlign: TextAlign.center,
      child: FutureBuilder<Industry?>(
        future: omatala
            .fetchIndustry(), // a previously-obtained Future<String> or null
        builder: (BuildContext context, AsyncSnapshot<Industry?> snapshot) {
          List<Widget> children;
          if (snapshot.hasData) {
            children = <Widget>[
              const Icon(
                Icons.check_circle_outline,
                color: Colors.green,
                size: 60,
              ),
              Padding(
                padding: const EdgeInsets.only(top: 16),
                child: const Text('Dashboard'),
              ),
              Padding(
                padding: const EdgeInsets.only(top: 16),
                child: Text('Industry: ${snapshot.data!.industry}'),
              ),
            ];
          } else if (snapshot.hasError) {
            children = <Widget>[
              const Icon(
                Icons.error_outline,
                color: Colors.red,
                size: 60,
              ),
              Padding(
                padding: const EdgeInsets.only(top: 16),
                child: Text('Error: ${snapshot.error}'),
              ),
            ];
          } else {
            children = const <Widget>[
              SizedBox(
                width: 60,
                height: 60,
                child: CircularProgressIndicator(),
              ),
              Padding(
                padding: EdgeInsets.only(top: 16),
                child: Text('Fetching data...'),
              ),
            ];
          }
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: children,
            ),
          );
        },
      ),
    );
  }
}
