# Documentation

## Writing JSDoc

Use following tags to create robust references using JSDoc.

- `@category` : Specifies the category in which the reference documentation will be placed. You can use `/` to specify depth (e.g. `Repository/Methods`).
- `@signature` : Describes what type of signature a function or method has. Please write a type declaration in the TypeScript language.
- `@param` : Write all parameters.
- `@returns` : Specifies what value is returned.
- `@throws` : Write documentation for errors that should be noted.
- `@example` : Examples are not required, but are recommended as they make the behavior easier to understand.

## Generating Reference

To automatically generate reference documentations after writing a JSDoc, please run the command below.

```shell
# Generate all reference documentations.
just gen-docs reference

# Generate all korean reference docuementations.
just gen-docs reference --lang=ko

# Generate specific reference documentations with glob pattern.
just gen-docs reference --pattern='Tree/Methods/*'
```
