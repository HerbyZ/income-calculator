# Storage module

Storage module is used to store models and options.

Models, serialized by serde_json, are stored in `storage.json` file. Each model has its own `StorageModel` placed in `sotrage::models` module. Storage models are needed to minimize storage file size, it means that all computable fields are not stored in file, instead they are calculated inside `StorageModel.to_model` function. To convert model to its storage model we have `StorageModel.from_model` function.

Main model is `Storage` defined in root of the module. To add new fields you need to modify `Storage` and `StorageModel` in `storage::models`. Methods `from_model` and `to_model` also have to be modified.

Methods `from_model` and `to_model` are provided by traits `FromModel` and `ToModel`.

Public module methods are stored in `storage` module root, there we have a basic interface to use storage: load/update. Also there is extended interface for loading and saving positions in `storage::positions` module.
