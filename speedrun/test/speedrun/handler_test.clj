(ns speedrun.handler-test
  (:require [clojure.test :refer :all]
            [ring.mock.request :as mock]
            [speedrun.handler :as h]
            [cheshire.core :as json]
            [org.httpkit.fake :refer [with-fake-http]]
            [speedrun.converters :as conv]))

(deftest test-routes
  (testing "wh route"
    (let [response (h/app (mock/request :post "/webhook"))]
      (is (= (:status response) 200))))

  (testing "not-found route"
    (let [response (h/app (mock/request :get "/invalid"))]
      (is (= (:status response) 404)))))

(def filename "test/example.json")

(def example-wh
  (json/parse-stream (clojure.java.io/reader filename) true))

(def req
  (-> (mock/request :post "/webhook")
      (mock/body (slurp filename))
      (mock/content-type "application/json")))

(deftest test-converters
  (testing "task converter"
    (let [issue {:id "329640334"
                 :name "Related"
                 :description "A task, related to #1 "
                 :creationDate "2018-06-05T21:58:53Z"
                 :changeDate "2018-06-19T21:20:27Z"
                 :status "open"
                 :creatorId 8050494
                 :handlerId 8050494
                 :url "https://github.com/CoproCodeForces-And-Friends/KFC-GitHub/issues/3"
                 :label ["swag" "yolo"]
                 :projectId 135080068
                 :activity []}]
      (is (= issue (conv/task example-wh)))))

  (testing "project converter"
    (let [repo {:id 135080068
                :name "KFC-GitHub"
                :description "GitHub issues connector for KFC project "
                :organizationId "33433403"
                :createDate "2018-05-27T20:40:51Z"
                :url "https://github.com/CoproCodeForces-And-Friends/KFC-GitHub"}]
      (is (= repo (conv/project example-wh)))))

  (testing "organization converter"
    (let [org {:id "33433403"
               :name "CoproCodeForces-And-Friends"
               :url "https://github.com/CoproCodeForces-And-Friends"}]
      (is (= org (conv/organization example-wh)))))

  (testing "participant"
    (let [part {:id "8050494"
                :name "fominok"
                :url "https://github.com/fominok"}]
      (is (= part (conv/participant example-wh))))))

(def good-storage (read-string (slurp "test/good_storage.edn")))

(deftest stateful-tests
  (def mock-storage (atom {}))
  (defn put [k]
    (fn [_ {:keys [body]} _]
      (swap! mock-storage update (keyword k) (fnil conj []) (json/decode body true))))
  (defn by-type [t] [(str "/storage/gh/" t) (put t)])

  (testing "Storage received all data converted from webhook"

    (with-fake-http (concat
                     (by-type "organization")
                     (by-type "project")
                     (by-type "user")
                     (by-type "task"))
      (h/app req)))

  (is (= @mock-storage good-storage)))
