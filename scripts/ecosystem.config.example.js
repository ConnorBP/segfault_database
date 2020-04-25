module.exports = {
    apps : [
        {
          name: "SegfaultDatabase",
          script: "./sfdb_graceful.sh",
          interpreter: "bash",
          kill_timeout : 31000,
          watch: true,
          env: {
              "DATABASE_URL": "mysql://segfault_users:password@127.0.0.1:3306/segfault_users",
          }
        }
    ]
}