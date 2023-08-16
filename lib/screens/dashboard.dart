import 'package:flutter/material.dart';
import 'package:kong/kong.dart';
import 'package:omatala/omatala.dart';
import 'package:bizkit/wid/menu.dart';
import 'package:bizkit/wid/scaffold.dart';

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
  static String navAddress = "/dashboard";

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
    Row title = Row(children: <Widget>[
      Container(
        margin: const EdgeInsets.only(right: 5),
        child: const Icon(Icons.cookie),
      ),
      const Text("BizKit | dashboard")
    ]);

    var industry = "";

    if (omatala.industry != null) {
      industry = omatala.industry!;
    }
    return bizkitScaffold(title, body(industry), bizkitMenu(context, industry));
  }
}

Widget body(String omatala) {
  return Padding(
    padding: const EdgeInsets.only(top: 16),
    child: Text('SME Industry: $omatala'),
  );
}
