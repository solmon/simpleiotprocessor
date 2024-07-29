use super::*;

pub async fn index() -> impl Responder {
    let data = serde_json::from_str::<ResponseData>(
        r#"
    {
    "response": {
      "code": "100",
      "text": "OK",
      "error": {
        "code": "",
        "text": "",
        "details": ""
      }
    },
    "app": {
        "id": "",
        "title": "",
        "icon": "",
        "template": "",
        "style": "",
        "session": "UUID",
        "lang": "",
        "navigation": [
          {
            "label": "Home",
            "icon": "ico_home",
            "tooltip": "",
            "target": {
              "host": "",
              "endpoint": "",
              "data": {}
            }
         },
         {
          "label": "Domains",
          "icon": "ico_list",
          "tooltip": "",
          "target": {
            "host": "",
            "endpoint": "",
            "data": {}
          }
        },
        {
          "label": "Logs",
          "icon": "ico_list",
          "tooltip": "",
          "target": {
            "host": "",
            "endpoint": "",
            "data": {}
          }
       },
       {
         "label": "Settings",
         "icon": "ico_maintenanceMachine.svg",
         "tooltip": "",
         "target": {
          "host": "",
          "endpoint": "",
          "data": {}
        }
      }        
        ],
        "toolbar": [],
        "menu": [],
        "data": {}
      },
    "display": {
      "style": "",
      "title": "Home Screen",
      "icon": "",
      "content": {
        "type": "section",
        "subtype": "",
        "style": "",
        "title": "",
        "label": "",
        "icon": "",
        "value": "",
        "data": {},
        "children": [
          {
            "type": "Text",
            "subtype": "",
            "style": "",
            "title": "",
            "label": "Name",
            "icon": "",
            "value": "Johnson",
            "data": {},
            "children": []
          },
          {
            "type": "Text",
            "subtype": "",
            "style": "",
            "title": "",
            "label": "Fist Name",
            "icon": "",
            "value": "John",
            "data": {},
            "children": []
          },
          {
            "type": "City",
            "subtype": "",
            "style": "",
            "title": "",
            "label": "Name",
            "icon": "",
            "value": "Johnsonville",
            "data": {},
            "children": []
          }
        ]
      },
      "menu:": [],
      "breadcrumbs": []
    },
    "actions": [],
    "records": {},
   "data": {}
  }
  "#,
    );

    web::Json(data.unwrap())
}
