int BAUD = 9600;
char MESSAGE[] = "ping";

void setup() {
  Serial.begin(BAUD);
}

void loop() {
  Serial.read();
  Serial.println(MESSAGE);
}
