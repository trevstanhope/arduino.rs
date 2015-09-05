int BAUD = 9600;
char MESSAGE[] = "{\"foo\":0,\"bar\":1}";
int LED_PIN = 13;
void setup() {
  Serial.begin(BAUD);
  pinMode(LED_PIN, OUTPUT);
}

void loop() {
  digitalWrite(LED_PIN, HIGH);
  if (Serial.available() > 0) {
    Serial.read();
    Serial.println(MESSAGE);
  }
  digitalWrite(LED_PIN, LOW);
}
