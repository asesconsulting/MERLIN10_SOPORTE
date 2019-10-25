<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiation_OK_Macro2</name>
   <tag></tag>
   <elementGuidId>89994f45-c222-45e7-ac15-0b5a8d398ee5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003creqFiliatorio\u003e\n  \u003cxfiliatorio\u003e\n    \u003ctipoDocumento\u003e\u003c/tipoDocumento\u003e\n    \u003cnumeroDocumento\u003e\u003c/numeroDocumento\u003e\n    \u003ctipoTributario\u003e\u003c/tipoTributario\u003e\n    \u003cnumeroTributario\u003e30517486678\u003c/numeroTributario\u003e\n    \u003capellido\u003e\u003c/apellido\u003e\n    \u003cnombre\u003e\u003c/nombre\u003e\n    \u003csexo\u003e\u003c/sexo\u003e\n    \u003cfechaNacimiento\u003e\u003c/fechaNacimiento\u003e\n\u003cclientAccessCode\u003e7d6bb345ce19a5c55627b8cb1f7f506b\u003c/clientAccessCode\u003e\n  \u003c/xfiliatorio\u003e\n\u003c/reqFiliatorio\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${RestService_1_5}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.RestService_1_5</defaultValue>
      <description></description>
      <id>4e2878f5-aea7-4e55-9365-10b5a094d43a</id>
      <masked>false</masked>
      <name>RestService_1_5</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


assertThat(response.getResponseText()).contains('&lt;estado>OK&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;tipoDocumento>&lt;/tipoDocumento>')
assertThat(response.getResponseText()).contains('&lt;numeroDocumento>0&lt;/numeroDocumento>')
assertThat(response.getResponseText()).contains('&lt;tipoTributario>80&lt;/tipoTributario>')
assertThat(response.getResponseText()).contains('&lt;numeroTributario>30517486678&lt;/numeroTributario>')
assertThat(response.getResponseText()).contains('&lt;apellido>&lt;/apellido>')
assertThat(response.getResponseText()).contains('&lt;nombre>BASF ARGENTINA S A&lt;/nombre>')
assertThat(response.getResponseText()).contains('&lt;sexo>&lt;/sexo>')
assertThat(response.getResponseText()).contains('&lt;fechaNacimiento>14/08/1969&lt;/fechaNacimiento>')
assertThat(response.getResponseText()).contains('&lt;marcaTributaria>C&lt;/marcaTributaria>')
assertThat(response.getResponseText()).contains('&lt;flgTipoDocumento>NA&lt;/flgTipoDocumento>')
assertThat(response.getResponseText()).contains('&lt;flgNumeroDocumento>NA&lt;/flgNumeroDocumento>')
assertThat(response.getResponseText()).contains('&lt;flgTipoTributario>AP&lt;/flgTipoTributario>')
assertThat(response.getResponseText()).contains('&lt;flgNumeroTributario>VA&lt;/flgNumeroTributario>')
assertThat(response.getResponseText()).contains('&lt;flgApellido>NI&lt;/flgApellido>')
assertThat(response.getResponseText()).contains('&lt;flgNombre>AP&lt;/flgNombre>')
assertThat(response.getResponseText()).contains('&lt;flgSexo>NA&lt;/flgSexo>')
assertThat(response.getResponseText()).contains('&lt;flgFechaNacimiento>AP&lt;/flgFechaNacimiento>')
assertThat(response.getResponseText()).contains('&lt;flgMarcaTributaria>&lt;/flgMarcaTributaria>')
assertThat(response.getResponseText()).contains('&lt;flgDistNombreApellido>&lt;/flgDistNombreApellido>')
assertThat(response.getResponseText()).contains('&lt;usuario>MERLIN&lt;/usuario>')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
