int BAUD = 9600;
char MESSAGE[] = "{\"foo\":0,\"bar\":1}";

void setup() {
  Serial.begin(BAUD);
  pinMode(13, OUTPUT);
}

void loop() {
  digitalWrite(13, HIGH);
  if (Serial.available() > 0) {
    Serial.read();
    Serial.println(MESSAGE);
  }
  digitalWrite(13, LOW);
}
