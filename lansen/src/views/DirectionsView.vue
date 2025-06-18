<script setup lang="ts">
import { Card } from 'primevue';
</script>
<template>
    <H1>Driving directions to Lansen Mold:</H1>
    <Card>
        <template #title>From the Mass Pike Rt 90, <em>for a car or light truck</em>:</template>
        <template #content>Mass Pike to exit 2, (Lee exit), to route 20 west and route 7 north. Follow route 7 north
            thru Lee, Lenox and into Pittsfield. On your left will be Pontoosuc lake, just after the lake you will enter
            Lanesborough. Stay on route 7 north about 2 more miles looking for small old cemetery on your right. Take
            the next right onto Summer St. Follow Summer St for about a mile until you hit a "T" in the road, a pond
            will be directly in front of you. Take a left, drive about 300 yards, Lansen Mold will be on your right.
        </template>
    </Card>
    <Card>
        <template #title>From the Mass Pike Rt 90, <em>for a large truck</em>:</template>
        <template #content>Mass Pike to exit 2, (Lee exit), to route 20 west and route 7 north. Follow route 7 north
            trough Lee, Lenox and into Pittsfield. On your left will be Pontoosuc lake, just after the lake you will
            enter Lanesborough. ? mile after the lake you will hit a traffic light, take a right onto Berkshire Mall Rd.
            Follow the Berkshire Mall Rd straight thru to route 8. Take a left onto route 8 north; stay on route 8 for
            about 2 miles. Watch for a small white church on your left. Just in front of the church is "Old State Rd",
            take the left onto "Old State Rd". Drive about 200 yards, Lansen Mold will be on your left.
        </template>
    </Card>
    <Card>
        <template #title>From points East on Route 2, <em>car or large truck</em>:</template>
        <template #content>Route 2 west into Charlemont. Take a left onto route 8A south. Take 8A south to route 116
            west, (right onto route 116). Follow route 116 into Adams, take a left on route 8 south. Follow route 8
            south, Cheshire Lake will be on your right. Just past Cheshire Lake you enter Lanesborough. After the sign
            "Entering Lanesborough" watch for a small white church on your right on the corner of route 8 and "Old State
            Rd". Take the right onto "Old State Rd". Lansen Mold is on the left about 200 yards down "Old State
            Rd".</template>
    </Card>
</template>
<style>
em {
     color: var(--p-red-900);
}
@media (prefers-color-scheme: dark) {
    em {
        color: var(--p-red-300);
    }
}
</style>