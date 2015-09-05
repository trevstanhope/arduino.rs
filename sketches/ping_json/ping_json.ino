int BAUD = 9600;
char MESSAGE[] = "{\"foo\":0,\"bar\":1}";

void setup() {
  Serial.begin(BAUD);
}

void loop() {
  if (Serial.available() > 0) {
    Serial.read();
    Serial.println(MESSAGE);
  }
}
