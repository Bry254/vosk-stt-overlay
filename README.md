# Real-Time Subtitle Overlay with Vosk

A simple overlay that uses [Vosk models](https://alphacephei.com/vosk/models) for real-time speech recognition and displays the recognized subtitles on screen.

## Usage

The program requires a Vosk model to perform speech recognition.

You can provide the path to the model as a command-line argument:

```bash
./vosk-tts-overlay /path/to/vosk-model
```

Alternatively, you can create a `model.txt` file in the same directory and write the path to the Vosk model inside it:

```text
/path/to/vosk-model
```

If no model argument is provided, the program will use the path specified in `model.txt`.

## Requirements

The required Vosk `.so` library must be located in the **same directory from which the program is executed**.

For example:

```text
vosk-tts-overlay/
├── vosk-tts-overlay
├── libvosk.so
└── model.txt
```

Make sure `libvosk.so` is available when running the program.
Can you download [here](https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip)

---

# Subtítulos en tiempo real con vosk

Un overlay sencillo que utiliza [modelos de Vosk](https://alphacephei.com/vosk/models) para reconocimiento de voz en tiempo real y muestra los subtítulos reconocidos en pantalla.

## Uso

El programa necesita un modelo de Vosk para realizar el reconocimiento de voz.

Puedes proporcionar la ruta al modelo como argumento:

```bash
./subtitle-overlay /ruta/al/modelo-de-vosk
```

También puedes crear un archivo `model.txt` en la misma carpeta y escribir dentro de él la ruta al modelo de Vosk:

```text
/ruta/al/modelo-de-vosk
```

Si no se proporciona un modelo como argumento, el programa utilizará la ruta especificada en `model.txt`.

## Requisitos

La biblioteca `.so` de Vosk debe encontrarse en la **misma carpeta desde la que se ejecuta el programa**.

Por ejemplo:

```text
vosk-tts-overlay/
├── vosk-tts-overlay
├── libvosk.so
└── model.txt
```

Asegúrate de que `libvosk.so` esté disponible al ejecutar el programa.
Puedes descargarlo [aqui](https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip)

## Licencia

Este proyecto está bajo la licencia MIT.
