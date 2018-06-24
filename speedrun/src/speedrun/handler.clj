(ns speedrun.handler
  (:require [compojure.core :refer :all]
            [compojure.route :as route]
            [speedrun.converters :as conv]
            [org.httpkit.client :as http]
            [environ.core :refer [env]]
            [cheshire.core :as json]
            [ring.middleware.json :refer [wrap-json-body]]
            [ring.middleware.defaults :refer [wrap-defaults site-defaults]]))

(def storage-url (env :storage-url))

(defn st-send [endpoint data]
  (http/post (str storage-url "/storage/gh" endpoint)
             {:body (json/generate-string data)}))

(defn wh-handler [req]
  (let [b (:body req)
        task (conv/task b)
        project (conv/project b)
        org (conv/organization b)
        pers (conv/participant b)
        futures
        [(st-send "/organization" org)
         (st-send "/project" project)
         (st-send "/user" pers)
         (st-send "/task" task)]]
    (dorun (map deref futures))
    {}))

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
