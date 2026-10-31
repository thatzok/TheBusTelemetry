# Handling The Bus Telemetry API Test Fixtures

Dieses Dokument beschreibt, wie standardisiert Testdaten aus "The Bus" API erzeugt werden um in die Tests integriert zu werden.

1. Das Programm "The Bus" starten und Telemetrie aktivieren
2. Die Castrop-Map starten und als Linie 341 mit Starthaltestelle Münsterplatz aufrufen, mit dem jeweiligen Bus dessen Testdaten abgefragt werden sollen
3. In den Bus einsteigen, auf den Fahrersitz setzen und die Zündung und den Motor einschalten, auf "D" stellen, aber die Türen nicht schließen
4. Mit dem Firefox-Browser die URL `http://127.0.0.1:37337/Vehicles/current` aufrufen
5. Die JSON-Antwort in tests/json/<busname>.json speichern
6. Schritte 2-5 für alle Busse aus tests/json/ abarbeiten
7. Tests laufen lassen um die neuen Testdaten zu verwenden und ggf. das Verarbeiten der Daten anpassen


Für den Mercedes-Benz Citaro O530 Diesel Voith muss das Mod https://steamcommunity.com/sharedfiles/filedetails/?id=3765388713 installiert sein
Für den MAN Lionscity A21 12M 2D Diesel muss das Mod https://steamcommunity.com/sharedfiles/filedetails/?id=3697187267 installiert sein

