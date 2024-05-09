
{} (:package |app)
  :configs $ {} (:init-fn |app.main/main!) (:reload-fn |app.main/reload!)
  :files $ {}
    |app.main $ %{} :FileEntry
      :defs $ {}
        |main! $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn main! () (println "|doing work") (+ 1 2)
        |reload! $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn reload! () $ println |TODO
      :ns $ %{} :CodeEntry (:doc |)
        :code $ quote
          ns app.main $ :require
