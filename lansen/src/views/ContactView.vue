<script setup lang="ts">
import GoForm from '@/components/GoForm.vue';
import { Card, Button, InputText, InputMask, Textarea, Message, Checkbox, CheckboxGroup } from 'primevue';
import { Form } from '@primevue/forms';
import { ref } from 'vue';

const mailLink = ref('info' + '@'.repeat(1) + 'lansen.com');
const initialValues = ref();
const resolver = ref();
const onFormSubmit = ({ valid }:{valid: boolean}) => {
    if (valid) {
        1;
    }
};
</script>
<template>
  <Card>
    <template #header>Contact Us</template>
    <template #content>
      <ul>
        <li>Driving directions to Lansen Mold. Separate instructions for large vehicles. <RouterLink to="/directions">Driving Directions
          </RouterLink>
        </li>
        <li>Reach us by phone. Call 413-443-5328 and ask for Kari Kristensen.
        </li>
        <li>Reach us via E-mail.
          <a v-bind:href="'mailto:' + mailLink">{{ mailLink }}</a>
        </li>
        <li>Request a quote. <GoForm /></li>
        <li>Our standard contact form. Fill out the contact page below and we will respond to your request.</li>
      </ul>

    </template>
  </Card>
  <Card>
    <template #header>Contact Form:</template>
    <template #content>
     <Form v-slot="$form" :resolver="resolver" :initialValues="initialValues" @submit="onFormSubmit" >
        <label for="name">Name</label>
        <InputText name="name" placeholder="Name" fluid />
        <label for="company">Company</label>
        <InputText name="company" placeholder="Company" fluid />
        <label for="email">Email</label>
        <InputText id="email" placeholder="email@domain.com" fluid />
        <label for="tel">Telephone</label>
        <InputMask id="tel" mask="999-999-9999" placeholder="413-555-5555" fluid />
        <label for="mail">Address</label><br>
        <Textarea id="mail" size="large" rows="5" placeholder="Business
1 Street Address
Berkshire, MA, 01224"></Textarea>
        <div class="flex flex-col gap-2 border-2">
          <label for="interests">Interests</label>
          <CheckboxGroup name="interests">
            <Checkbox inputId="custom" value="custom" />
            <label for="custom"> Custom Molds </label>
            <Checkbox inputId="inject" value="inject" />
            <label for="inject"> Injection Molding </label>
            <Checkbox inputId="specimen" value="specimen" />
            <label for="specimen"> Specimen Molding </label>
            <Checkbox inputId="plaque" value="plaque" />
            <label for="plaque"> Plaque Molding </label>
            <Checkbox inputId="other" value="other" />
            <label for="other"> Other </label>
          </CheckboxGroup>
        </div>
        <div class="flex flex-col gap-2">
          <label for="additional">Comments Or Questions</label><br>
          <Textarea id="mail" size="large" rows="5" placeholder="Additional Details"></Textarea>
        </div>
        <Button type="submit" severity="secondary" label="Submit" fluid></Button>
      </Form>
    </template>
  </Card>
</template>