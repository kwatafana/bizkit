/// Product creation input
class ProductInput {
    constructor(name, manufacturer, barcode, unit, weight, price, description, photos_input, category){
	this.name = name;
	this.manufacturer = manufacturer,
	this.barcode = barcode,
	this.unit = unit,
	this.weight = weight,
	this.price = price,
	this.description = description,
	this.photos_input = photos_input,
	this.category = category
    }

    /// Validate input
    validate(){
	// TODO:
    }
}

/// errors
const ProductError = {
    InvalidInput: Error('Invalid user input'),
    InternalServer: Error('Internal server error'),
    NotFound: Error('Resource does not exist'),
    Unauthorized: Error('Unauthorized, please login')
};

const ProductAPI = {
    address: "/products",

    /// Submit category
    async add(input) {
	if (!input instanceof ProductInput){
	    throw ProductError.InvalidInput;
	}

	// validate input
	input.validate();

	const formData = new FormData();
	formData.append("name", input.name);
	formData.append("manufacturer", input.manufacturer);
	formData.append("barcode", input.barcode);
	formData.append("description", input.description);

	if(input.unit){
	    formData.append("unit", input.unit);
	}

	if(input.weight){
	    formData.append("weight", input.weight);
	}

	if(input.price){
	    formData.append("price", input.price);
	}

	if(input.category){
	    let  category = parseInt(input.category);
	    formData.append("category", input.category);
	}

	for (const [i, photo] of
	     Array.from(input.photos_input.files).entries()){
 	    formData.append(`photo_${i}`, photo);
	}

	return fetch(this.address,{
	    method: "POST",
	    body: formData,
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw ProductError.InvalidInput;
		case 401:
		    throw ProductError.Unauthorized;
		case 500:
		    throw ProductError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
    /// Get all categories
    async get_all() {
	return fetch(this.address, {
	    method: "GET",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw ProductError.InvalidInput;
		case 401:
		    throw ProductError.Unauthorized;
		case 404:
		    throw ProductError.NotFound;
		case 500:
		    throw ProductError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
    /// Get single product by id
    async get_by_id(id) {
	return fetch(this.address+"/"+id, {
	    method: "GET",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw ProductError.InvalidInput;
		case 401:
		    throw ProductError.Unauthorized;
		case 404:
		    throw ProductError.NotFound;
		case 500:
		    throw ProductError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
    /// Delete single blog post by id
    async delete_by_id(id) {
	return fetch(this.address+"/"+id, {
	    method: "DELETE",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return;
		case 400:
		    throw ProductError.InvalidInput;
		case 401:
		    throw ProductError.Unauthorized;
		case 404:
		    throw ProductError.NotFound;
		case 500:
		    throw ProductError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    }

};
