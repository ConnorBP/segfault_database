module.exports = {
    apps : [
        {
          name: "SegfaultDatabase",
          script: "./sfdb_run.sh",
          watch: true,
          env: {
              "DATABASE_URL": "mysql://segfault_users:password@127.0.0.1:3306/segfault_users",
          }
        }
    ]
}