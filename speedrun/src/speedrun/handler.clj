(ns speedrun.handler
  (:require [compojure.core :refer :all]
            [compojure.route :as route]
            [ring.middleware.json :refer [wrap-json-body]]
            [ring.middleware.defaults :refer [wrap-defaults site-defaults]]))

(defn wh-handler [req]
  (prn (:body req))
  {:yolo "swag"})

(defroutes app-routes
  (POST "/webhook" req (wh-handler req))
  (route/not-found "Not Found"))

(def app
  (wrap-json-body
   (wrap-defaults
    app-routes
    (assoc-in site-defaults
              [:security :anti-forgery] false))
   {:keywords? true}))
