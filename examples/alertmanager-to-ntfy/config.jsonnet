/*
  Convert Alertmanager webhook to ntfy format

  Alertmanager JSON: https://prometheus.io/docs/alerting/latest/configuration/#webhook_config
  ntfy JSON: https://docs.ntfy.sh/publish/#publish-as-json
*/

{
  "topic": "mytopic",
  "title": body.alerts[0].labels.alertname, // TODO support multiple alerts
  "message": body.alerts[0].annotations.description,
  "tags": [],
  "priority": 3,
  "actions": []
}
