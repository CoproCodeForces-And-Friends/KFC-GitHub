(ns speedrun.converters)

(defn task [wh]
  (let [i (:issue wh)
        r (:repository wh)]
    {:id (str (:id i))
     :name (:title i)
     :description (:body i)
     :creationDate (:created_at i)
     :changeDate (:updated_at i)
     :status (:state i)
     :creatorId (get-in i [:user :id])
     :handlerId (get-in i [:assignee :id])
     :url (:html_url i)
     :label (mapv :name (:labels i))
     :projectId (:id r)
     :activity []}))

(defn project [wh]
  (let [r (:repository wh)
        o (:owner r)]
    {:id (:id r)
     :name (:name r)
     :description (:description r)
     :organizationId (when (= "Organization" (:type o)) (str (:id o)))
     :createDate (:created_at r)
     :url (:html_url r)}))

(defn organization [wh]
  (let [o (get-in wh [:repository :owner])]
    {:id (str (:id o))
     :name (:login o)
     :url (:html_url o)}))

(defn participant [wh]
  (let [ass (get-in wh [:issue :assignee])]
    {:id (str (:id ass))
     :name (:login ass)
     :url (:html_url ass)}))
