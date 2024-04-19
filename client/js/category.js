/// Course creation input
class CategoryInput {
    constructor(name){
	this.name = name;
    }

    /// Validate input
    validate(){
	// TODO:
    }
}

/// errors
const CategoryError = {
    InvalidInput: Error('Invalid user input'),
    InternalServer: Error('Internal server error'),
    NotFound: Error('Resource does not exist'),
    Unauthorized: Error('Unauthorized, please login')
};

const CategoryAPI = {
    address: "/categories",

    /// Submit category
    async add_category(input) {
	if (!input instanceof CategoryInput){
	    throw CategoryError.InvalidInput;
	}

	// validate input
	input.validate();

	return fetch(this.address,{
	    method: "POST",
	    headers: {
		"Content-Type": "application/json",
	    },
	    body: JSON.stringify(input),
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw CategoryError.InvalidInput;
		case 401:
		    throw CategoryError.InvalidInput;
		case 500:
		    throw CategoryError.InternalServer;
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
		    throw CategoryError.InvalidInput;
		case 401:
		    throw CategoryError.Unauthorized;
		case 404:
		    throw CategoryError.NotFound;
		case 500:
		    throw CategoryError.InternalServer;
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
		    throw CategoryError.InvalidInput;
		case 401:
		    throw CategoryError.Unauthorized;
		case 404:
		    throw CategoryError.NotFound;
		case 500:
		    throw CategoryError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    }

};
