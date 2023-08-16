import 'package:image_picker/image_picker.dart';

List<String> extractFilePaths(List<XFile> files) {
  List<String> extracts = [];
  for (var i = 0; i < files.length; i++) {
    extracts.add(files[i].path);
  }
  return extracts;
}
