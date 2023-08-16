import 'package:flutter/material.dart';
import 'package:bizkit/wid/scaffold.dart';
import 'package:kong/kong.dart';
import 'package:bizkit/wid/menu.dart';
import 'package:omatala/omatala.dart';
import 'package:image_picker/image_picker.dart';
import 'package:bizkit/util.dart';
import 'package:bizkit/error.dart';

/// Add Property Screen
class AddPropertyScreen extends StatefulWidget {
  const AddPropertyScreen(
      {Key? key,
      required this.title,
      required this.kong,
      required this.omatala})
      : super(key: key);
  final KongAPI kong;
  final OmatalaAPI omatala;
  final String title;
  static String navAddress = "/add-property";

  @override
  State<AddPropertyScreen> createState() =>
      _AddPropertyScreenState(kong: kong, omatala: omatala);
}

class _AddPropertyScreenState extends State<AddPropertyScreen> {
  _AddPropertyScreenState({required this.kong, required this.omatala});
  final KongAPI kong;
  final OmatalaAPI omatala;
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    Row title = Row(children: <Widget>[
      Container(
        margin: const EdgeInsets.only(right: 5),
        child: const Icon(Icons.cookie),
      ),
      const Text("BizKit | add property")
    ]);
    return bizkitScaffold(
        title, body(context, kong, omatala), bizkitMenu(context, ""));
  }
}

Widget body(context, kong, omatala) {
  TextEditingController propertyNameController = TextEditingController();
  TextEditingController numberOfBedroomsController = TextEditingController();
  TextEditingController numberOfBathroomsController = TextEditingController();
  TextEditingController propertySqftController = TextEditingController();
  TextEditingController propertyAddressController = TextEditingController();
  TextEditingController propertyAgentIdController = TextEditingController();
  TextEditingController propertyDescriptionController = TextEditingController();
  TextEditingController propertyPriceController = TextEditingController();
  List<XFile> propertyImages = [];

  return ListView(children: <Widget>[
    Container(
      margin: const EdgeInsets.only(right: 5),
      child: const Icon(Icons.lock_open, color: Colors.white),
    ),
    const Text(
      "Add Property",
      style: TextStyle(
        color: Colors.white,
        fontSize: 20,
      ),
    ),
    Container(
      padding: const EdgeInsets.all(5),
      child: const Divider(color: Colors.tealAccent),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: propertyNameController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Name",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: numberOfBedroomsController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Number of Bedrooms",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        keyboardType: TextInputType.number,
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: numberOfBathroomsController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Number of Bathrooms",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        keyboardType: TextInputType.number,
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: propertySqftController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Property size (sqft)",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        keyboardType: TextInputType.number,
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: propertyAddressController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Property Address",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: propertyAgentIdController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Property Agent",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        keyboardType: TextInputType.number,
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: propertyDescriptionController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Describe Property",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: propertyPriceController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Price of property (optional)",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        keyboardType: TextInputType.number,
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
        height: 50,
        padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
        margin: const EdgeInsets.only(top: 10.0),
        child: ElevatedButton(
            child: const Text("Pick Property Photos"),
            onPressed: () async {
              final ImagePicker picker = ImagePicker();
              // Pick multiple images.
              propertyImages = await picker.pickMultiImage();
            })),
    Container(
        height: 50,
        padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
        margin: const EdgeInsets.only(top: 10.0),
        child: ElevatedButton(
          child: const Text("Add Property"),
          onPressed: () async {
            final propertyName = propertyNameController.text;
            final numberOfBedrooms = numberOfBedroomsController.text;
            final numberOfBathrooms = numberOfBathroomsController.text;
            final propertySqft = propertySqftController.text;
            final propertyAddress = propertyAddressController.text;
            final propertyAgent = propertyAgentIdController.text;
            final propertyDescription = propertyDescriptionController.text;
            String? propertyPrice;

            if (propertyPriceController.text.isEmpty) {
              propertyPrice = null;
            } else {
              propertyPrice = propertyPriceController.text;
            }

            login(
                propertyName,
                numberOfBedrooms,
                numberOfBathrooms,
                propertySqft,
                propertyAddress,
                propertyAgent,
                propertyDescription,
                propertyPrice,
                propertyImages,
                kong,
                context);
          },
        )),
  ]);
}

// Send login request and if login succeeds fetch node data
login(
    String propertyName,
    String numberOfBedrooms,
    String numberOfBathrooms,
    String propertySqft,
    String propertyAddress,
    String propertyAgent,
    String propertyDescription,
    String? propertyPrice,
    List<XFile> propertyImages,
    KongAPI kong,
    context) async {
  if (propertyName.isNotEmpty &&
      numberOfBedrooms.isNotEmpty &&
      numberOfBathrooms.isNotEmpty &&
      propertySqft.isNotEmpty &&
      propertyAddress.isNotEmpty &&
      propertyAgent.isNotEmpty &&
      propertyDescription.isNotEmpty &&
      propertyImages.isNotEmpty) {
    try {
      final photos = extractFilePaths(propertyImages);
      double? price;

      if (propertyPrice != null) {
        price = double.parse(propertyPrice);
      }

      final input = PropertyCreationInput(
          propertyName,
          int.parse(numberOfBedrooms),
          int.parse(numberOfBathrooms),
          double.parse(propertySqft),
          propertyAddress,
          int.parse(propertyAgent),
          propertyDescription,
          price,
          photos);

      if (kong.propertiesAPI == null) {
        BizkitError.properties_api_not_set;
      }

      kong.propertiesAPI!.postProperty(input);
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(e.toString())),
      );
    }
  } else {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
          content: Text('Fill in all fields, only the price can be left out.')),
    );
  }
}
